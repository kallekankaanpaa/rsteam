use crate::utils::{Error, ErrorKind, ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/IPlayerService/GetCommunityBadgeProgress/v0001/";

#[derive(Deserialize)]
pub struct Quest {
    pub questid: u32,
    pub completed: bool,
}

#[derive(Deserialize)]
struct Quests {
    quests: Vec<Quest>,
}

type Response = ResponseWrapper<Quests>;

impl SteamClient {
    /// Resturns the current community badge process for user
    pub async fn get_community_badge_progress(&self, id: SteamID) -> Result<Vec<Quest>> {
        let api_key = self
            .api_key()
            .map_err(|_| Error::new(ErrorKind::APIKeyRequired))?;

        let query = format!("key={}&steamid={}", api_key, id);

        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let response: Response = from_slice(&to_bytes(raw_response.into_body()).await?)?;

        Ok(response.response.quests)
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
        let quests = block_on(client.get_community_badge_progress(id)).unwrap();
        assert_eq!(quests.len(), 28)
    }
}
