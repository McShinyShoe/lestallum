// SPDX-License-Identifier: GPL-3.0-or-later

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::api::{ApiError, build_url, get_json};

const GEYSER: &str = "geysermc";
const GEYSER_BASE: &str = "https://api.geysermc.org";

const MOJANG: &str = "mojang";
const MOJANG_BASE: &str = "https://api.mojang.com";
const MOJANG_SESSION_BASE: &str = "https://sessionserver.mojang.com";

const MAX_USERNAME_CHARS: usize = 16;
const MAX_GAMERTAG_CHARS: usize = 16;

const TEXTURES_PROPERTY: &str = "textures";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountLink {
    pub bedrock_id: u64,
    pub java_id: Uuid,
    pub java_name: String,
    pub last_name_update: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerTextures {
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MinecraftController {
    http: Client,
}

impl MinecraftController {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    pub async fn link_from_xuid(&self, xuid: u64) -> Result<Option<AccountLink>, ApiError> {
        let xuid = xuid.to_string();
        let url = build_url(GEYSER, GEYSER_BASE, &["v2", "link", "bedrock", &xuid])?;
        let value: Value = get_json(&self.http, GEYSER, url, &xuid).await?;

        optional_link(value)
    }

    pub async fn link_from_uuid(&self, uuid: Uuid) -> Result<Option<AccountLink>, ApiError> {
        let uuid = uuid.hyphenated().to_string();
        let url = build_url(GEYSER, GEYSER_BASE, &["v2", "link", "java", &uuid])?;
        let links: Vec<AccountLink> = get_json(&self.http, GEYSER, url, &uuid).await?;

        Ok(links.into_iter().next())
    }

    pub async fn xuid_from_gamertag(&self, gamertag: &str) -> Result<u64, ApiError> {
        let gamertag = validated_gamertag(gamertag)?;
        let url = build_url(GEYSER, GEYSER_BASE, &["v2", "xbox", "xuid", gamertag])?;
        let result: XuidResult = get_json(&self.http, GEYSER, url, gamertag).await?;

        Ok(result.xuid)
    }

    pub async fn gamertag_from_xuid(&self, xuid: u64) -> Result<String, ApiError> {
        let xuid = xuid.to_string();
        let url = build_url(GEYSER, GEYSER_BASE, &["v2", "xbox", "gamertag", &xuid])?;
        let result: GamertagResult = get_json(&self.http, GEYSER, url, &xuid).await?;

        Ok(result.gamertag)
    }

    pub async fn uuid_from_username(&self, username: &str) -> Result<Uuid, ApiError> {
        let username = validated_username(username)?;
        let url = build_url(
            MOJANG,
            MOJANG_BASE,
            &["users", "profiles", "minecraft", username],
        )?;
        let profile: MojangProfile = get_json(&self.http, MOJANG, url, username).await?;

        Ok(profile.id)
    }

    pub async fn username_from_uuid(&self, uuid: Uuid) -> Result<String, ApiError> {
        Ok(self.session_profile(uuid).await?.name)
    }

    pub async fn textures_from_uuid(&self, uuid: Uuid) -> Result<PlayerTextures, ApiError> {
        let profile = self.session_profile(uuid).await?;

        let Some(property) = profile
            .properties
            .into_iter()
            .find(|property| property.name == TEXTURES_PROPERTY)
        else {
            return Ok(PlayerTextures::default());
        };

        decode_textures(&property.value)
    }

    async fn session_profile(&self, uuid: Uuid) -> Result<SessionProfile, ApiError> {
        let uuid = uuid.simple().to_string();
        let url = build_url(
            MOJANG,
            MOJANG_SESSION_BASE,
            &["session", "minecraft", "profile", &uuid],
        )?;

        get_json(&self.http, MOJANG, url, &uuid).await
    }
}

#[derive(Debug, Deserialize)]
struct XuidResult {
    xuid: u64,
}

#[derive(Debug, Deserialize)]
struct GamertagResult {
    gamertag: String,
}

#[derive(Debug, Deserialize)]
struct MojangProfile {
    id: Uuid,
}

#[derive(Debug, Deserialize)]
struct SessionProfile {
    name: String,
    #[serde(default)]
    properties: Vec<SessionProperty>,
}

#[derive(Debug, Deserialize)]
struct SessionProperty {
    name: String,
    value: String,
}

#[derive(Debug, Default, Deserialize)]
struct TexturePayload {
    #[serde(default)]
    textures: TextureSet,
}

#[derive(Debug, Default, Deserialize)]
struct TextureSet {
    #[serde(rename = "SKIN")]
    skin: Option<TextureEntry>,
    #[serde(rename = "CAPE")]
    cape: Option<TextureEntry>,
}

#[derive(Debug, Deserialize)]
struct TextureEntry {
    url: String,
}

fn optional_link(value: Value) -> Result<Option<AccountLink>, ApiError> {
    if value.as_object().is_some_and(Map::is_empty) {
        return Ok(None);
    }

    serde_json::from_value(value)
        .map(Some)
        .map_err(|source| ApiError::decode(GEYSER, source))
}

fn decode_textures(encoded: &str) -> Result<PlayerTextures, ApiError> {
    let decoded = BASE64
        .decode(encoded)
        .map_err(|source| ApiError::decode(MOJANG, source))?;

    let payload: TexturePayload =
        serde_json::from_slice(&decoded).map_err(|source| ApiError::decode(MOJANG, source))?;

    Ok(PlayerTextures {
        skin_url: payload.textures.skin.map(|entry| entry.url),
        cape_url: payload.textures.cape.map(|entry| entry.url),
    })
}

fn validated_username(username: &str) -> Result<&str, ApiError> {
    let accepted = (1..=MAX_USERNAME_CHARS).contains(&username.len())
        && username
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_');

    if accepted {
        return Ok(username);
    }

    Err(ApiError::Input {
        service: MOJANG,
        reason: format!(
            "username must be 1 to {MAX_USERNAME_CHARS} characters of a-z, A-Z, 0-9 or _"
        ),
    })
}

fn validated_gamertag(gamertag: &str) -> Result<&str, ApiError> {
    let accepted = (1..=MAX_GAMERTAG_CHARS).contains(&gamertag.chars().count())
        && !gamertag.chars().any(char::is_control);

    if accepted {
        return Ok(gamertag);
    }

    Err(ApiError::Input {
        service: GEYSER,
        reason: format!("gamertag must be 1 to {MAX_GAMERTAG_CHARS} printable characters"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTCH_TEXTURES: &str = "ewogICJ0aW1lc3RhbXAiIDogMTc4NTIwMTIxMTM2NCwKICAicHJvZmlsZUlkIiA6ICIwNjlhNzlmNDQ0ZTk0NzI2YTViZWZjYTkwZTM4YWFmNSIsCiAgInByb2ZpbGVOYW1lIiA6ICJOb3RjaCIsCiAgInRleHR1cmVzIiA6IHsKICAgICJTS0lOIiA6IHsKICAgICAgInVybCIgOiAiaHR0cDovL3RleHR1cmVzLm1pbmVjcmFmdC5uZXQvdGV4dHVyZS8yOTIwMDlhNDkyNWI1OGYwMmM3N2RhZGMzZWNlZjA3ZWE0Yzc0NzJmNjRlMGZkYzMyY2U1NTIyNDg5MzYyNjgwIgogICAgfQogIH0KfQ==";

    #[test]
    fn empty_object_means_the_xuid_is_not_linked() {
        assert_eq!(optional_link(serde_json::json!({})).unwrap(), None);
    }

    #[test]
    fn a_populated_object_parses_into_a_link() {
        let value = serde_json::json!({
            "bedrock_id": 2535432196048835u64,
            "java_id": "d34eb447-6e90-4c78-9281-600df88aef1d",
            "java_name": "Tim203",
            "last_name_update": 1664541698215i64,
        });

        let link = optional_link(value).unwrap().unwrap();

        assert_eq!(link.bedrock_id, 2535432196048835);
        assert_eq!(link.java_name, "Tim203");
    }

    #[test]
    fn textures_decode_to_a_skin_url_without_a_cape() {
        let textures = decode_textures(NOTCH_TEXTURES).unwrap();

        assert_eq!(
            textures.skin_url.as_deref(),
            Some(
                "http://textures.minecraft.net/texture/292009a4925b58f02c77dadc3ecef07ea4c7472f64e0fdc32ce5522489362680"
            )
        );
        assert_eq!(textures.cape_url, None);
    }

    #[test]
    fn usernames_are_bounded_before_they_reach_the_url() {
        assert!(validated_username("Notch").is_ok());
        assert!(validated_username("_a1").is_ok());
        assert!(validated_username("").is_err());
        assert!(validated_username("seventeen_chars17").is_err());
        assert!(validated_username("../../etc/passwd").is_err());
        assert!(validated_username("Notch?query=1").is_err());
    }

    #[test]
    fn gamertags_allow_spaces_but_not_control_characters() {
        assert!(validated_gamertag("Some Gamer").is_ok());
        assert!(validated_gamertag("").is_err());
        assert!(validated_gamertag("this tag is far too long").is_err());
        assert!(validated_gamertag("bad\nname").is_err());
    }
}
