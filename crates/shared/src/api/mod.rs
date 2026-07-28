pub mod minecraft;
pub mod skin;

use std::sync::Once;
use std::time::Duration;

use bytes::{Bytes, BytesMut};
use reqwest::{Client, Response, StatusCode, Url};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::api::minecraft::MinecraftController;
use crate::api::skin::SkinController;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_JSON_BYTES: usize = 64 * 1024;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("could not build the http client")]
    Client(#[source] reqwest::Error),

    #[error("{service}: {reason}")]
    Input {
        service: &'static str,
        reason: String,
    },

    #[error("{service}: request failed")]
    Transport {
        service: &'static str,
        #[source]
        source: reqwest::Error,
    },

    #[error("{service}: responded {status}: {message}")]
    Upstream {
        service: &'static str,
        status: StatusCode,
        message: String,
    },

    #[error("{service}: no result for {subject}")]
    NotFound {
        service: &'static str,
        subject: String,
    },

    #[error("{service}: could not decode the response")]
    Decode {
        service: &'static str,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("{service}: response exceeded {limit} bytes")]
    TooLarge { service: &'static str, limit: usize },
}

impl ApiError {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }

    fn decode(
        service: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Decode {
            service,
            source: Box::new(source),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiController {
    pub minecraft: MinecraftController,
    pub skin: SkinController,
}

impl ApiController {
    pub fn new() -> Result<Self, ApiError> {
        install_crypto_provider();

        let http = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .map_err(ApiError::Client)?;

        Ok(Self::from_client(http))
    }

    pub fn from_client(http: Client) -> Self {
        Self {
            minecraft: MinecraftController::new(http.clone()),
            skin: SkinController::new(http),
        }
    }
}

fn install_crypto_provider() {
    static INSTALLED: Once = Once::new();

    INSTALLED.call_once(|| {
        if rustls::crypto::ring::default_provider()
            .install_default()
            .is_err()
        {
            tracing::debug!("a rustls crypto provider was already installed");
        }
    });
}

pub(crate) fn parse_url(service: &'static str, raw: &str) -> Result<Url, ApiError> {
    Url::parse(raw).map_err(|_| ApiError::Input {
        service,
        reason: format!("`{raw}` is not a usable url"),
    })
}

pub(crate) fn build_url(
    service: &'static str,
    base: &str,
    segments: &[&str],
) -> Result<Url, ApiError> {
    let mut url = parse_url(service, base)?;

    {
        let mut path = url.path_segments_mut().map_err(|_| ApiError::Input {
            service,
            reason: format!("`{base}` cannot be a base url"),
        })?;

        for segment in segments {
            path.push(segment);
        }
    }

    Ok(url)
}

pub(crate) async fn get_json<T>(
    http: &Client,
    service: &'static str,
    url: Url,
    subject: &str,
) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let response = send(http, service, url).await?;
    let status = response.status();
    let body = read_bounded(service, response, MAX_JSON_BYTES).await?;

    if status == StatusCode::NOT_FOUND {
        return Err(ApiError::NotFound {
            service,
            subject: subject.to_owned(),
        });
    }

    if !status.is_success() {
        return Err(ApiError::Upstream {
            service,
            status,
            message: upstream_message(&body),
        });
    }

    if body.is_empty() {
        return Err(ApiError::NotFound {
            service,
            subject: subject.to_owned(),
        });
    }

    serde_json::from_slice(&body).map_err(|source| ApiError::decode(service, source))
}

pub(crate) async fn send(
    http: &Client,
    service: &'static str,
    url: Url,
) -> Result<Response, ApiError> {
    http.get(url)
        .send()
        .await
        .map_err(|source| ApiError::Transport { service, source })
}

pub(crate) async fn read_bounded(
    service: &'static str,
    mut response: Response,
    limit: usize,
) -> Result<Bytes, ApiError> {
    let mut body = BytesMut::new();

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|source| ApiError::Transport { service, source })?
    {
        if body.len() + chunk.len() > limit {
            return Err(ApiError::TooLarge { service, limit });
        }

        body.extend_from_slice(&chunk);
    }

    Ok(body.freeze())
}

pub(crate) fn upstream_message(body: &[u8]) -> String {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(body) else {
        return "no message".to_owned();
    };

    value
        .get("message")
        .or_else(|| value.get("errorMessage"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("no message")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_appends_segments_to_a_bare_host() {
        let url = build_url("test", "https://example.org", &["v2", "link", "java"]).unwrap();

        assert_eq!(url.as_str(), "https://example.org/v2/link/java");
    }

    #[test]
    fn build_url_percent_encodes_segments() {
        let url = build_url("test", "https://example.org", &["xbox", "a/../b c"]).unwrap();

        assert_eq!(url.as_str(), "https://example.org/xbox/a%2F..%2Fb%20c");
    }

    #[test]
    fn upstream_message_reads_both_error_shapes() {
        assert_eq!(upstream_message(br#"{"message":"nope"}"#), "nope");
        assert_eq!(upstream_message(br#"{"errorMessage":"gone"}"#), "gone");
        assert_eq!(upstream_message(b"<html>"), "no message");
    }
}
