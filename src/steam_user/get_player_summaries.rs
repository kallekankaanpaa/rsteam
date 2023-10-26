use std::net::Ipv4Addr;

use crate::client::SteamClient;
use crate::error::Error;
use crate::steam_id::SteamID;
use crate::utils::{
    PlayersWrapper, ResponseWrapper, Result, AUTHORITY,
};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetPlayerSummaries/v0002/";

#[derive(Debug, Default, Deserialize)]
#[serde(from = "u32")]
pub enum CommentPermission {
    #[default]
    FriendsOnly = 0,
    Public = 1,
    Private = 2,
}

impl From<u32> for CommentPermission {
    fn from(c: u32) -> Self {
        match c {
            0 => CommentPermission::FriendsOnly,
            1 => CommentPermission::Public,
            2 => CommentPermission::Private,
            _ => CommentPermission::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(from = "u32")]
pub enum Visibility {
    #[default]
    Private = 1,
    Visible = 3,
}

impl From<u32> for Visibility {
    fn from(v: u32) -> Self {
        match v {
            1 => Visibility::Private,
            3 => Visibility::Visible,
            _ => Visibility::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(from = "u32")]
pub enum Status {
    #[default]
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
            _ => Status::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(from = "u32")]
pub enum ProfileState {
    #[default]
    Unconfigured = 0,
    Configured = 1,
}

impl From<u32> for ProfileState {
    fn from(value: u32) -> Self {
        match value {
            0 => ProfileState::Unconfigured,
            1 => ProfileState::Configured,
            _ => ProfileState::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    /// Steam ID of the user
    #[serde(rename = "steamid")]
    pub id: SteamID,
    /// Visibility of the profile
    /// 
    /// Visibility is determined from the viewpoint of the API key owner. 
    #[serde(rename = "communityvisibilitystate")]
    pub visibility: Visibility,
    #[serde(rename = "profilestate")]
    pub profile_state: ProfileState, 
    #[serde(rename = "personaname")]
    pub profile_name: String,
    /// Unix timestamp of users last logoff
    /// 
    /// Only available for Steam users that are friends of the user whose API key is used
    #[serde(rename = "lastlogoff")]
    pub last_logoff: Option<u32>,
    #[serde(rename = "profileurl")]
    pub profile_url: String,
    /// 32x32 pixel image
    pub avatar: String,
    /// 64x64 pixel image
    #[serde(rename = "avatarmedium")]
    pub avatar_medium: String,
    /// 184x184 pixel image
    #[serde(rename = "avatarfull")]
    pub avatar_full: String,
    #[serde(rename = "personastate")]
    pub status: Status,
    #[serde(default)]
    #[serde(rename = "commentpermission")]
    pub comment_permission: CommentPermission,
    #[serde(rename = "realname")]
    pub real_name: Option<String>,
    #[serde(rename = "primaryclanid")]
    pub primary_clan_id: Option<SteamID>,
    /// Unix timestamp of users creation
    #[serde(rename = "timecreated")]
    pub time_created: Option<u32>,
    #[serde(rename = "loccountrycode")]
    pub country_code: Option<String>,
    #[serde(rename = "loccityid")]
    pub city_id: Option<u32>,
    #[serde(rename = "gameid")]
    pub game_id: Option<String>,
    #[serde(rename = "gameextrainfo")]
    pub game_info: Option<String>,
    #[serde(rename = "gameserveerip")]
    pub gameserver_ip: Option<Ipv4Addr>,
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
    pub async fn get_player_summaries(&self, ids: &[SteamID]) -> Result<Vec<Summary>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        if ids.len() > 100 {
            return Err(Error::client("too many IDs (> 100)"));
        }
        let id_query = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let query = format!("key={api_key}&steamids={id_query}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
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
            client.get_player_summaries(&vec![SteamID::from(76561198061271782)]),
        )
        .unwrap();
        println!("{:?}", summary);
        assert!(summary.len() == 1);
    }

    #[test]
    fn works_with_multiple() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let summary = tokio_test::block_on(client.get_player_summaries(&vec![
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
            client.get_player_summaries(&vec![SteamID::from(7656119806127178)]),
        )
        .unwrap();
        assert!(summary.is_empty());
    }
}
