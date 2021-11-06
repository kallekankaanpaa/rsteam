use std::num::NonZeroU32;

use crate::error::Error;
use crate::utils::{Result, AUTHORITY};
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/ISteamUserStats/GetUserStatsForGame/v0002";

#[derive(Deserialize)]
pub struct Stat {
    pub name: String,
    pub value: u32,
}

#[derive(Deserialize)]
pub struct Achievement {
    pub name: String,
    pub achieved: u32,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct RawPlayerStats {
    steamID: String,
    gameName: String,
    stats: Vec<Stat>,
    achievements: Vec<Achievement>,
}

#[derive(Deserialize)]
struct Response {
    playerstats: RawPlayerStats,
}

pub struct PlayerStats {
    pub id: SteamID,
    pub game_name: String,
    pub stats: Vec<Stat>,
    pub achievements: Vec<Achievement>,
}

impl From<RawPlayerStats> for PlayerStats {
    fn from(stats: RawPlayerStats) -> Self {
        PlayerStats {
            id: stats.steamID.parse::<u64>().unwrap().into(),
            game_name: stats.gameName,
            stats: stats.stats,
            achievements: stats.achievements,
        }
    }
}

impl SteamClient {
    /// Gets users [PlayerStats] for given game
    ///
    /// Requires an API key.
    pub async fn get_user_stats_for_game(
        &self,
        id: SteamID,
        game_id: NonZeroU32,
    ) -> Result<PlayerStats> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or(Error::Client("API key required".to_owned()))?;
        //.map_err(|_| Error::new(ErrorKind::APIKeyRequired))?;
        let query = format!("key={}&steamid={}&appid={}", api_key, id, game_id);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let response = self.client.get(uri).await?;
        let body = response.into_body();
        let parsed = from_slice::<Response>(&to_bytes(body).await?)?;

        Ok(parsed.playerstats.into())
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
            tokio_test::block_on(client.get_user_stats_for_game(id, game_id)).unwrap();
        assert!(!player_stats.stats.is_empty());
        assert!(!player_stats.achievements.is_empty());
    }
}
