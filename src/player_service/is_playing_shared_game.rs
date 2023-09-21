use crate::error::Error;
use crate::utils::{ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/IPlayerService/IsPlayingSharedGame/v0001/";

#[derive(Deserialize)]
struct Lender {
    lender_steamid: Option<String>,
}

type Response = ResponseWrapper<Lender>;

impl SteamClient {
    /// Returns the lenders steamid if user is playing shared game
    pub async fn is_playing_shared_game(
        &self,
        id: &SteamID,
        appid: u32,
    ) -> Result<Option<SteamID>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let query = format!("key={}&steamid={}&appid={}", api_key, id, appid);

        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let response: Response = from_slice(&to_bytes(raw_response.into_body()).await?)?;

        let Lender { lender_steamid } = response.response;

        if let Some(steamid) = lender_steamid {
            Ok(Some(
                steamid
                    .parse::<u64>()
                    .map_err(|_| {
                        Error::Client("request succeeded but lenders steamid is invalid".to_owned())
                    })?
                    .into(),
            ))
        } else {
            Ok(None)
        }
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
        let lender = block_on(client.is_playing_shared_game(&id, 730)).unwrap();
        assert_eq!(lender, None)
    }
}
