use std::fmt;
use std::str::FromStr;

use crate::client::SteamClient;
use crate::steam_id::SteamID;
use crate::utils::{Error, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetFriendList/v1";

#[derive(Debug, PartialEq)]
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
        write!(f, "{}", repr)
    }
}

impl FromStr for Relation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "friend" => Ok(Relation::Friend),
            _ => Ok(Relation::All),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Friend {
    pub id: SteamID,
    pub relationship: Relation,
    pub friend_since: u32,
}

impl From<RawFriend> for Friend {
    fn from(rf: RawFriend) -> Self {
        Friend {
            id: rf.steamid.parse::<u64>().unwrap().into(),
            relationship: rf.relationship.parse::<Relation>().unwrap(),
            friend_since: rf.friend_since,
        }
    }
}

#[derive(Deserialize, Debug)]
struct RawFriend {
    steamid: String,
    relationship: String,
    friend_since: u32,
}

#[derive(Deserialize)]
struct FriendsWrapper {
    friends: Vec<RawFriend>,
}

#[derive(Deserialize)]
struct FriendList {
    friendslist: FriendsWrapper,
}

impl SteamClient {
    /// Returns a vector of [Friends](Friend) for the provided [SteamID]
    pub async fn get_friend_list(
        &self,
        id: SteamID,
        relationship: Option<Relation>,
    ) -> Result<Vec<Friend>> {
        let api_key = self.api_key.as_ref().ok_or(Error {
            cause: "resolve_vanity_url requires an api key".to_owned(),
        })?;
        let relation = match relationship {
            Some(relation) => format!("&relationship={}", relation.to_string()),
            None => "".to_owned(),
        };
        let query = format!("key={}&steamid={}{}", api_key, id, relation);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;
        let response = self.client.get(uri).await;
        let body = response?.into_body();
        let friendlist = serde_json::from_slice::<FriendList>(&to_bytes(body).await?)?.friendslist;

        Ok(friendlist.friends.into_iter().map(|f| f.into()).collect())
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
            tokio_test::block_on(client.get_friend_list(SteamID::from(76561198061271782), None))
                .unwrap();
        assert!(!friends.is_empty());
    }
}
