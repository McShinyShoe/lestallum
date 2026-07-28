// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt;

use bytes::Bytes;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use uuid::Uuid;

use crate::api::{ApiError, parse_url, read_bounded, send, upstream_message};

const MCHEADS: &str = "mc-heads";
const MCHEADS_BASE: &str = "https://mc-heads.net";

const FLAT_MIN_SIZE: u32 = 8;
const FLAT_MAX_SIZE: u32 = 600;
const ISOMETRIC_MIN_SIZE: u32 = 32;
const ISOMETRIC_MAX_SIZE: u32 = 600;

const MAX_IMAGE_BYTES: usize = 2 * 1024 * 1024;
const IMAGE_CONTENT_TYPE_PREFIX: &str = "image/";
const DEFAULT_CONTENT_TYPE: &str = "image/png";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkinImage {
    pub bytes: Bytes,
    pub content_type: String,
}

pub fn simple_url(uuid: Uuid, size: u32, helm: bool) -> String {
    let size = size.clamp(FLAT_MIN_SIZE, FLAT_MAX_SIZE);
    let uuid = uuid.simple();

    if helm {
        format!("{MCHEADS_BASE}/avatar/{uuid}/{size}")
    } else {
        format!("{MCHEADS_BASE}/avatar/{uuid}/{size}/nohelm")
    }
}

pub fn isometric_url(uuid: Uuid, size: u32, direction: Direction) -> String {
    let size = size.clamp(ISOMETRIC_MIN_SIZE, ISOMETRIC_MAX_SIZE);

    format!("{MCHEADS_BASE}/body/{}/{size}/{direction}", uuid.simple())
}

pub fn full_body_url(uuid: Uuid, size: u32) -> String {
    let size = size.clamp(FLAT_MIN_SIZE, FLAT_MAX_SIZE);

    format!("{MCHEADS_BASE}/player/{}/{size}", uuid.simple())
}

pub fn combo_url(uuid: Uuid, size: u32) -> String {
    let size = size.clamp(FLAT_MIN_SIZE, FLAT_MAX_SIZE);

    format!("{MCHEADS_BASE}/combo/{}/{size}", uuid.simple())
}

pub fn texture_url(uuid: Uuid) -> String {
    format!("{MCHEADS_BASE}/skin/{}", uuid.simple())
}

#[derive(Debug, Clone)]
pub struct SkinController {
    http: Client,
}

impl SkinController {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    pub async fn simple(&self, uuid: Uuid, size: u32, helm: bool) -> Result<SkinImage, ApiError> {
        self.fetch(simple_url(uuid, size, helm), uuid).await
    }

    pub async fn isometric(
        &self,
        uuid: Uuid,
        size: u32,
        direction: Direction,
    ) -> Result<SkinImage, ApiError> {
        self.fetch(isometric_url(uuid, size, direction), uuid).await
    }

    pub async fn full_body(&self, uuid: Uuid, size: u32) -> Result<SkinImage, ApiError> {
        self.fetch(full_body_url(uuid, size), uuid).await
    }

    pub async fn combo(&self, uuid: Uuid, size: u32) -> Result<SkinImage, ApiError> {
        self.fetch(combo_url(uuid, size), uuid).await
    }

    pub async fn texture(&self, uuid: Uuid) -> Result<SkinImage, ApiError> {
        self.fetch(texture_url(uuid), uuid).await
    }

    async fn fetch(&self, url: String, uuid: Uuid) -> Result<SkinImage, ApiError> {
        let url = parse_url(MCHEADS, &url)?;
        let response = send(&self.http, MCHEADS, url).await?;
        let status = response.status();

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or(DEFAULT_CONTENT_TYPE)
            .to_owned();

        let bytes = read_bounded(MCHEADS, response, MAX_IMAGE_BYTES).await?;

        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound {
                service: MCHEADS,
                subject: uuid.simple().to_string(),
            });
        }

        if !status.is_success() {
            return Err(ApiError::Upstream {
                service: MCHEADS,
                status,
                message: upstream_message(&bytes),
            });
        }

        if !content_type.starts_with(IMAGE_CONTENT_TYPE_PREFIX) {
            return Err(ApiError::Upstream {
                service: MCHEADS,
                status,
                message: format!("expected an image, got `{content_type}`"),
            });
        }

        Ok(SkinImage {
            bytes,
            content_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTCH: Uuid = Uuid::from_u128(0x069a79f4_44e9_4726_a5be_fca90e38aaf5);

    #[test]
    fn simple_urls_carry_the_helm_modifier() {
        assert_eq!(
            simple_url(NOTCH, 100, true),
            "https://mc-heads.net/avatar/069a79f444e94726a5befca90e38aaf5/100"
        );
        assert_eq!(
            simple_url(NOTCH, 100, false),
            "https://mc-heads.net/avatar/069a79f444e94726a5befca90e38aaf5/100/nohelm"
        );
    }

    #[test]
    fn isometric_urls_carry_the_direction() {
        assert_eq!(
            isometric_url(NOTCH, 128, Direction::Left),
            "https://mc-heads.net/body/069a79f444e94726a5befca90e38aaf5/128/left"
        );
        assert_eq!(
            isometric_url(NOTCH, 128, Direction::Right),
            "https://mc-heads.net/body/069a79f444e94726a5befca90e38aaf5/128/right"
        );
    }

    #[test]
    fn full_body_and_combo_and_texture_urls_match_the_documented_routes() {
        assert_eq!(
            full_body_url(NOTCH, 64),
            "https://mc-heads.net/player/069a79f444e94726a5befca90e38aaf5/64"
        );
        assert_eq!(
            combo_url(NOTCH, 64),
            "https://mc-heads.net/combo/069a79f444e94726a5befca90e38aaf5/64"
        );
        assert_eq!(
            texture_url(NOTCH),
            "https://mc-heads.net/skin/069a79f444e94726a5befca90e38aaf5"
        );
    }

    #[test]
    fn sizes_are_clamped_to_what_the_service_accepts() {
        assert!(simple_url(NOTCH, 0, true).ends_with("/8"));
        assert!(simple_url(NOTCH, 9000, true).ends_with("/600"));
        assert!(isometric_url(NOTCH, 1, Direction::Left).contains("/32/"));
        assert!(isometric_url(NOTCH, 9000, Direction::Left).contains("/600/"));
    }
}
