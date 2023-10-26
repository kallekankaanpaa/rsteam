use std::fmt;

use crate::client::SteamClient;
use crate::error::Error;
use crate::steam_id::SteamID;
use crate::utils::{Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetFriendList/v1";

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Relation {
    Friend,
    All,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Relation::Friend => "friend",
            Relation::All => "other",
        };
        write!(f, "{repr}")
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Friend {
    #[serde(rename = "steamid")]
    pub id: SteamID,
    pub relationship: Relation,
    pub friend_since: u32,
}

#[derive(Deserialize)]
struct FriendsWrapper {
    friends: Vec<Friend>,
}

#[derive(Deserialize)]
struct FriendList {
    friendslist: Option<FriendsWrapper>,
}

impl SteamClient {
    /// Returns a vector of [Friends](Friend) for the provided [SteamID]
    ///
    /// Requires an API key.
    pub async fn get_friend_list(
        &self,
        id: &SteamID,
        relationship: Option<Relation>,
    ) -> Result<Vec<Friend>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let relation = optional_query!(relationship);
        let query = format!("key={api_key}&steamid={id}{relation}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
            .build()?;
        let response = self.client.get(uri).await;
        let body = response?.into_body();
        let friendlist = serde_json::from_slice::<FriendList>(&to_bytes(body).await?)?.friendslist;

        Ok(friendlist.map(|fl| fl.friends).unwrap_or(vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn works() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let friends =
            tokio_test::block_on(client.get_friend_list(&SteamID::from(76561198061271782), None))
                .unwrap();

        for f in &friends {
            println!("{f:?}");
        }

        assert!(!friends.is_empty());
    }
}
