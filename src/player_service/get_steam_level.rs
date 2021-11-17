use crate::error::Error;
use crate::utils::{ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/IPlayerService/GetSteamLevel/v0001";

#[derive(Deserialize)]
struct Level {
    player_level: u32,
}

type Response = ResponseWrapper<Level>;

impl SteamClient {
    /// Returns users steam level
    pub async fn get_steam_level(&self, id: &SteamID) -> Result<u32> {
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
        let raw_body = raw_response.into_body();
        let response: Response = from_slice(&to_bytes(raw_body).await?)?;

        Ok(response.response.player_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio_test::block_on;

    #[test]
    fn get_level() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = SteamID::from(76561198061271782);
        let level = block_on(client.get_steam_level(&id)).unwrap();
        assert_eq!(level, 36);
    }
}
