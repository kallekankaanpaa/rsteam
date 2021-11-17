use std::num::NonZeroU32;

use crate::error::Error;
use crate::utils::{Result, AUTHORITY};
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/ISteamUserStats/GetUserStatsForGame/v0002";

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub name: String,
    pub value: u32,
}

#[derive(Deserialize, Debug)]
pub struct Achievement {
    pub name: String,
    pub achieved: u32,
}

#[derive(Deserialize)]
struct Response {
    playerstats: PlayerStats,
}

#[derive(Deserialize)]
pub struct PlayerStats {
    #[serde(rename = "steamID")]
    pub id: SteamID,
    #[serde(rename = "gameName")]
    pub game_name: String,
    pub stats: Vec<Stat>,
    pub achievements: Vec<Achievement>,
}

impl SteamClient {
    /// Gets users [PlayerStats] for given game
    ///
    /// Requires an API key.
    pub async fn get_user_stats_for_game(
        &self,
        id: &SteamID,
        game_id: NonZeroU32,
    ) -> Result<PlayerStats> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let query = format!("key={}&steamid={}&appid={}", api_key, id, game_id);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let response = self.client.get(uri).await?;
        let body = response.into_body();
        let parsed = from_slice::<Response>(&to_bytes(body).await?)?;

        Ok(parsed.playerstats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn fetch_stats() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let game_id = NonZeroU32::new(730).unwrap();
        let player_stats =
            tokio_test::block_on(client.get_user_stats_for_game(&id, game_id)).unwrap();

        for stat in &player_stats.stats {
            println!("{:?}", stat)
        }
        for achievement in &player_stats.achievements {
            println!("{:?}", achievement)
        }
        assert!(!player_stats.stats.is_empty());
        assert!(!player_stats.achievements.is_empty());
    }
}
