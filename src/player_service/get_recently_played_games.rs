use crate::utils::{Error, ErrorKind, ResponseWrapper, Result, AUTHORITY};
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
    pub img_logo_url: String,
    pub playtime_windows_forever: u32,
    pub playtime_mac_forever: u32,
    pub playtime_linux_forever: u32,
}

#[derive(Deserialize, Debug)]
pub struct RecentlyPlayedGames {
    pub total_count: u32,
    pub games: Vec<Game>,
}

type Response = ResponseWrapper<RecentlyPlayedGames>;

impl SteamClient {
    pub async fn get_recently_played_games(
        &self,
        id: SteamID,
        count: Option<u32>,
    ) -> Result<RecentlyPlayedGames> {
        let api_key = self
            .api_key()
            .map_err(|_| Error::new(ErrorKind::APIKeyRequired))?;
        let count_query = count
            .map(|c| format!("&count={}", c))
            .unwrap_or("".to_owned());

        let query = format!("key={}&steamid={}{}", api_key, id, count_query);
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
    //use std::env;
    use tokio_test::{assert_err, block_on};

    #[test]
    fn asfd() {
        let client = SteamClient::new();
        let id = SteamID::from(76561198061271782);
        let recent = block_on(client.get_recently_played_games(id, None));
        assert_err!(recent);
    }
}
