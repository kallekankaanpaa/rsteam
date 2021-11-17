use crate::error::Error;
use crate::utils::{ResponseWrapper, Result, AUTHORITY};
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::Uri;

use serde::Deserialize;
use serde_json::from_slice;

const PATH: &str = "/ISteamUser/GetUserGroupList/v1";

#[derive(Deserialize)]
struct Group {
    gid: String,
}

#[derive(Deserialize)]
struct Resp {
    success: bool,
    groups: Vec<Group>,
}

type Response = ResponseWrapper<Resp>;

impl SteamClient {
    /// Fetches vector of [SteamIDs](SteamID) which represent the ids for the users groups
    ///
    /// Requires an API key.
    pub async fn get_user_group_list(&self, id: &SteamID) -> Result<Vec<SteamID>> {
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

        let response = self.client.get(uri).await?;
        let body = response.into_body();
        let parsed = from_slice::<Response>(&to_bytes(body).await?)?;

        let Resp { success, groups } = parsed.response;

        if success {
            Ok(groups
                .into_iter()
                .filter_map(|g| g.gid.parse::<u64>().ok())
                .map(|n| n.into())
                .collect::<Vec<SteamID>>())
        } else {
            Err(Error::Client("request failed".to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn fetches_list_of_ids() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let groups =
            tokio_test::block_on(client.get_user_group_list(&SteamID::from(76561198061271782)))
                .unwrap();
        assert!(!groups.is_empty());
    }
}
