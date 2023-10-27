use crate::error::Error;
use crate::utils::{Result, AUTHORITY};
use crate::SteamClient;

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_json::from_slice;

const PATH: &str = "/ISteamUserStats/GetSchemaForGame/v0002";

#[derive(Deserialize, Debug)]
pub struct Achievement {
    pub name: String,
    #[serde(rename = "defaultvalue")]
    pub default_value: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub hidden: bool,
    pub description: Option<String>,
    pub icon: String,
    #[serde(rename = "icongray")]
    pub icon_gray: String,
}

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub name: String,
    #[serde(rename = "defaultvalue")]
    pub default_value: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct AvailableGameStats {
    #[serde(default)]
    pub stats: Vec<Stat>,
    #[serde(default)]
    pub achievements: Vec<Achievement>,
}

#[derive(Deserialize, Debug)]
struct GameInner {
    #[serde(rename = "gameName")]
    game_name: String,
    #[serde(rename = "gameVersion")]
    game_version: String,
    #[serde(rename = "availableGameStats")]
    available_game_stats: AvailableGameStats,
}

#[derive(Debug)]
pub struct Game {
    pub game_name: String,
    pub game_version: String,
    pub stats: Vec<Stat>,
    pub achievements: Vec<Achievement>,
}

impl From<GameInner> for Game {
    fn from(value: GameInner) -> Self {
        Self {
            game_name: value.game_name,
            game_version: value.game_version,
            stats: value.available_game_stats.stats,
            achievements: value.available_game_stats.achievements,
        }
    }
}

#[derive(Deserialize)]
struct Response {
    game: GameInner,
}

impl SteamClient {
    pub async fn get_schema_for_game(
        &self,
        app_id: u32,
        language: Option<&str>,
    ) -> Result<Game> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let lang = optional_query!(language, "l");

        let query = format!("key={api_key}&appid={app_id}{lang}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
            .build()?;

        let response = self.client.get(uri).await?;
        let body = response.into_body();
        let Response { game } = from_slice::<Response>(&to_bytes(body).await?)?;

        Ok(game.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn get_cs2_schema() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let app_id = 730;
        let cs2_schema =
            tokio_test::block_on(client.get_schema_for_game(app_id, None)).unwrap();

        assert_eq!(cs2_schema.game_name, "ValveTestApp260");
        assert_eq!(cs2_schema.achievements.len(), 1);
        assert_eq!(cs2_schema.stats.len(), 286);
    }

    #[test]
    fn get_starfield_schema() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let app_id = 1716740;
        let starfield_schema =
            tokio_test::block_on(client.get_schema_for_game(app_id, None)).unwrap();
        
        assert_eq!(starfield_schema.game_name, "Starfield");
        assert_eq!(starfield_schema.achievements.len(), 50);
        assert_eq!(starfield_schema.stats.len(), 0);
    }

    #[test]
    fn language_option() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let app_id = 730;
        let german_schema =
            tokio_test::block_on(client.get_schema_for_game(app_id, Some("german")))
                .unwrap();

        assert!(german_schema.achievements
            .iter()
            .any(|a| a.display_name == "Ein neuer Anfang"))
    }
}
