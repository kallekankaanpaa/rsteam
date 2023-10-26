use crate::error::Error;
use crate::utils::{ResponseMaybeEmpty, Result, AUTHORITY};
use crate::{SteamClient, SteamID};
use serde::Deserialize;
use serde_json::from_slice;

use hyper::body::to_bytes;
use hyper::Uri;

const PATH: &str = "/IPlayerService/GetRecentlyPlayedGames/v0001/";

/// The playtimes are in minutes and platform specific playtimes
/// may not add up to playtime_forever.
#[derive(Deserialize, Debug)]
pub struct Game {
    pub appid: u32,
    pub name: String,
    pub playtime_2weeks: u32,
    pub playtime_forever: u32,
    pub img_icon_url: String,
    pub playtime_windows_forever: u32,
    pub playtime_mac_forever: u32,
    pub playtime_linux_forever: u32,
}

#[derive(Deserialize, Debug)]
pub struct RecentlyPlayedGames {
    pub total_count: u32,
    pub games: Vec<Game>,
}

type Response = ResponseMaybeEmpty<RecentlyPlayedGames>;

impl SteamClient {
    /// Returns info about users recently played games.
    ///
    /// Length of games vector can be limited by settings the optional
    /// count parameter. By default there is no limit.
    pub async fn get_recently_played_games(
        &self,
        id: &SteamID,
        count: Option<u32>,
    ) -> Result<Option<RecentlyPlayedGames>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let count_query = optional_query!(count);

        let query = format!("key={api_key}&steamid={id}{count_query}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response = from_slice(&to_bytes(raw_body).await?)?;

        Ok(response.response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio_test::{block_on, assert_ok};

    #[test]
    fn public_profile() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let recent = block_on(client.get_recently_played_games(&id, None));
        println!("{recent:?}");
        assert_ok!(recent);
    }

    #[test]
    fn private_profile() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198312831106);
        let recent = block_on(client.get_recently_played_games(&id, None));
        println!("{recent:?}");
        assert_ok!(recent);
    }
}
