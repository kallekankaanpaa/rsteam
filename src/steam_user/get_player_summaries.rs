use std::net::Ipv4Addr;

use crate::client::SteamClient;
use crate::error::Error;
use crate::steam_id::SteamID;
use crate::utils::{
    bool_from_int_maybe_missing, concat_steam_ids, PlayersWrapper, ResponseWrapper, Result,
    AUTHORITY,
};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetPlayerSummaries/v0002/";

#[derive(Debug, Deserialize)]
#[serde(from = "u32")]
pub enum Visibility {
    Private = 1,
    FriendsOnly = 2,
    FriendsOfFriends = 3,
    UsersOnly = 4,
    Public = 5,
}

impl From<u32> for Visibility {
    fn from(v: u32) -> Self {
        match v {
            1 => Visibility::Private,
            2 => Visibility::FriendsOnly,
            3 => Visibility::FriendsOfFriends,
            4 => Visibility::UsersOnly,
            5 => Visibility::Public,
            _ => Visibility::Private,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "u32")]
pub enum Status {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
}

impl From<u32> for Status {
    fn from(status: u32) -> Self {
        match status {
            0 => Status::Offline,
            1 => Status::Online,
            2 => Status::Busy,
            3 => Status::Away,
            4 => Status::Snooze,
            5 => Status::LookingToTrade,
            6 => Status::LookingToPlay,
            _ => Status::Offline,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    #[serde(rename = "steamid")]
    pub id: SteamID,
    #[serde(rename = "communityvisibilitystate")]
    pub visibility: Visibility,
    /// Profile state, 1 means user has configured the profile
    #[serde(rename = "profilestate")]
    pub profile_state: u8,
    #[serde(rename = "personaname")]
    pub profile_name: String,
    /// Unix timestamp of users last logoff
    #[serde(rename = "lastlogoff")]
    pub last_logoff: u64,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    /// 32x32 pixel image
    pub avatar: String,
    /// 64x64 pixel image
    pub avatarmedium: String,
    /// 184x184 pixel image
    pub avatarfull: String,
    #[serde(rename = "personastate")]
    pub status: Status,
    #[serde(default)]
    #[serde(rename = "commentpermission")]
    #[serde(deserialize_with = "bool_from_int_maybe_missing")]
    pub comment_permission: Option<bool>,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    pub primaryclanid: Option<SteamID>,
    /// Unix timestamp of users creation
    #[serde(rename = "timecreated")]
    pub time_created: Option<u64>,
    #[serde(rename = "loccountrycode")]
    pub country_code: Option<String>,
    #[serde(rename = "loccityid")]
    pub city_id: Option<u64>,
    #[serde(rename = "gameid")]
    pub game_id: Option<String>,
    #[serde(rename = "gameextrainfo")]
    pub game_info: Option<String>,
    #[serde(rename = "gameserveerip")]
    pub gameserver_ip: Option<Ipv4Addr>,
}

#[derive(Deserialize, Debug)]
struct RawSummary {
    steamid: String,
    communityvisibilitystate: u8,
    profilestate: u8,
    personaname: String,
    lastlogoff: u64,
    profileurl: String,
    avatar: String,
    avatarmedium: String,
    avatarfull: String,
    personastate: u8,
    commentpermission: Option<u8>,
    realname: Option<String>,
    primaryclanid: Option<String>,
    timecreated: Option<u64>,
    loccountrycode: Option<String>,
    loccityid: Option<u64>,
    gameid: Option<String>,
    gameextrainfo: Option<String>,
    gameserverip: Option<String>,
}

/// Private Response type to simplify these utility types
type Response = ResponseWrapper<PlayersWrapper<Summary>>;

impl SteamClient {
    /// Gets vector of player/account [Summaries](Summary).
    ///
    /// Requires an API key and works with maximum of 100 [SteamIDs](SteamID).
    /// If the [SteamID] is invalid or user doesn't exist with the ID the API
    /// just drops the summary from the response. So don't assume the returned
    /// [Summaries](Summary) are in the same order as the [SteamIDs](SteamID).
    /// Always check the [SteamID] from the [Summary] struct.
    pub async fn get_player_summaries(&self, ids: Vec<SteamID>) -> Result<Vec<Summary>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        if ids.len() > 100 {
            return Err(Error::client("too many IDs (> 100)"));
        }
        let id_query = concat_steam_ids(ids);
        let query = format!("key={}&steamids={}", api_key, id_query);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let response = self.client.get(uri).await;
        let body = response?.into_body();
        let resp = serde_json::from_slice::<Response>(&to_bytes(body).await?)?.response;

        Ok(resp.players)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn works_with_single() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let summary = tokio_test::block_on(
            client.get_player_summaries(vec![SteamID::from(76561198061271782)]),
        )
        .unwrap();
        println!("{:?}", summary);
        assert!(summary.len() == 1);
    }

    #[test]
    fn works_with_multiple() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let summary = tokio_test::block_on(client.get_player_summaries(vec![
            SteamID::from(76561198061271782),
            SteamID::from(76561198072766352),
        ]))
        .unwrap();
        assert!(summary.len() == 2);
    }

    #[test]
    fn works_with_invalid() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let summary = tokio_test::block_on(
            client.get_player_summaries(vec![SteamID::from(7656119806127178)]),
        )
        .unwrap();
        assert!(summary.is_empty());
    }
}
