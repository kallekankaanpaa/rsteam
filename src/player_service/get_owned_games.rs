use crate::utils::{format_query_param, Error, ErrorKind, ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};
use serde::Deserialize;
use serde_json::from_slice;

use hyper::body::to_bytes;
use hyper::Uri;

const PATH: &str = "/IPlayerService/GetOwnedGames/v0001/";

/// The playtimes are in minutes and platform specific playtimes
/// may not add up to playtime_forever.
#[derive(Deserialize, Debug)]
pub struct Game {
    pub appid: u32,
    pub name: Option<String>,
    pub playtime_forever: u32,
    pub img_icon_url: Option<String>,
    pub img_logo_url: Option<String>,
    pub playtime_windows_forever: u32,
    pub playtime_mac_forever: u32,
    pub playtime_linux_forever: u32,
}

#[derive(Deserialize, Debug)]
pub struct OwnedGames {
    pub game_count: u32,
    pub games: Vec<Game>,
}

type Response = ResponseWrapper<OwnedGames>;

impl SteamClient {
    pub async fn get_owned_games(
        &self,
        id: SteamID,
        // appid_filter: Option<u32>, in API documentation but no couldn't find any way to use
        include_app_info: Option<bool>,          // default false
        include_played_free_games: Option<bool>, // default false
        include_free_sub: Option<bool>,          // default false
        skip_unvetted_apps: Option<bool>,        // default false?
    ) -> Result<OwnedGames> {
        let api_key = self
            .api_key()
            .map_err(|_| Error::new(ErrorKind::APIKeyRequired))?;

        let q1 = format_query_param(include_app_info, "include_appinfo");
        let q2 = format_query_param(include_played_free_games, "include_played_free_games");
        let q3 = format_query_param(include_free_sub, "include_free_sub");
        let q4 = format_query_param(skip_unvetted_apps, "skip_unvetted_apps");

        let query = format!("key={}&steamid={}{}{}{}{}", api_key, id, q1, q2, q3, q4);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
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
    use tokio_test::block_on;

    #[test]
    fn asfd() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let owned_games = block_on(client.get_owned_games(id, None, None, None, None)).unwrap();
        assert_eq!(owned_games.game_count, 70);
    }
}