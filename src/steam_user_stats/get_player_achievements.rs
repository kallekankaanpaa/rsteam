use crate::error::Error;
use crate::utils::{Result, AUTHORITY};
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;
use serde_aux::field_attributes::deserialize_bool_from_anything;

const PATH: &str = "/ISteamUserStats/GetPlayerAchievements/v0001";

#[derive(Deserialize, Debug)]
pub struct Achievement {
    #[serde(rename = "apiname")]
    pub api_name: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub achieved: bool,
    #[serde(rename = "unlocktime")]
    pub unlock_time: u32,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    success: bool,
    #[serde(default)]
    achievements: Vec<Achievement>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct Response {
    playerstats: Metadata,
}

impl SteamClient {
    /// Gets users [`PlayerStats`] for given game
    ///
    /// Requires an API key.
    pub async fn get_player_achievements(
        &self,
        id: &SteamID,
        app_id: u32,
        language: Option<&str>,
    ) -> Result<Vec<Achievement>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let lang = optional_query!(language, "l");

        let query = format!("key={api_key}&steamid={id}&appid={app_id}{lang}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
            .build()?;
        
        let response = self.client.get(uri).await?;
        let body = response.into_body();
        let Response { playerstats } = from_slice::<Response>(&to_bytes(body).await?)?;

        if playerstats.success {
            Ok(playerstats.achievements)
        } else {
            Err(Error::client(
                &playerstats
                    .error
                    .unwrap_or("Something went wrong".to_string()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn get_achievements() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let app_id = 730;
        let achievements =
            tokio_test::block_on(client.get_player_achievements(&id, app_id, None))
                .unwrap();
        assert!(achievements.iter().any(|a| a.api_name == "PLAY_CS2".to_string()))
    }

    #[test]
    fn language_option() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let app_id = 730;
        let achievements =
            tokio_test::block_on(client.get_player_achievements(&id, app_id, Some("german")))
                .unwrap();

        assert!(achievements.iter().any(|a| a.name == Some("Ein neuer Anfang".to_string())))
    }
}
