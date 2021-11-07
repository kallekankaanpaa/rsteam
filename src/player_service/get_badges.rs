use crate::error::Error;
use crate::utils::{ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/IPlayerService/GetBadges/v0001/";

#[derive(Deserialize)]
pub struct Badge {
    pub badgeid: u32,
    pub level: u32,
    pub completion_time: u32,
    pub communityitemid: Option<String>,
    pub border_color: Option<u32>,
    pub appid: Option<u32>,
    pub xp: u32,
    pub scarcity: u32,
}

#[derive(Deserialize)]
pub struct Badges {
    pub badges: Vec<Badge>,
    pub player_xp: u32,
    pub player_level: u32,
    pub player_xp_needed_to_level_up: u32,
    pub player_xp_needed_current_level: u32,
}

type Response = ResponseWrapper<Badges>;

impl SteamClient {
    /// Returns all badges user has and info about level
    pub async fn get_badges(&self, id: SteamID) -> Result<Badges> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let query = format!("key={}&steamid={}", api_key, id);

        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let response: Response = from_slice(&to_bytes(raw_response.into_body()).await?)?;

        Ok(response.response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio_test::block_on;

    #[test]
    fn badges() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let badges = block_on(client.get_badges(id)).unwrap();
        assert_eq!(badges.badges.len(), 19)
    }
}
