use crate::client::SteamClient;
use crate::steam_id::SteamID;
use crate::utils::{concat_steam_ids, Error, PlayersWrapper, ResponseWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetPlayerSummaries/v0002/";

#[derive(Debug)]
pub enum Visibility {
    Private = 1,
    FriendsOnly = 2,
    FriendsOfFriends = 3,
    UsersOnly = 4,
    Public = 5,
}

impl From<u8> for Visibility {
    fn from(v: u8) -> Self {
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

#[derive(Debug)]
pub enum Status {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
    Snooze = 4,
    LookingToTrade = 5,
    LookingToPlay = 6,
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
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

#[derive(Debug)]
pub struct Summary {
    pub id: SteamID,
    pub visibility: Visibility,
    pub profile_state: u8,
    pub profile_name: String,
    pub last_logoff: u64,
    pub profile_url: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub status: Status,
    pub comment_permission: Option<bool>,
    pub real_name: Option<String>,
    pub primaryclanid: Option<String>,
    pub time_created: Option<u64>,
    pub country_code: Option<String>,
    pub city_id: Option<u64>,
    pub game_id: Option<String>,
    pub game_info: Option<String>,
    pub gameserver_ip: Option<String>,
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

impl From<RawSummary> for Summary {
    fn from(rs: RawSummary) -> Self {
        Summary {
            id: rs.steamid.parse::<u64>().unwrap().into(),
            visibility: rs.communityvisibilitystate.into(),
            profile_state: rs.profilestate,
            profile_name: rs.personaname,
            last_logoff: rs.lastlogoff,
            profile_url: rs.profileurl,
            avatar: rs.avatar,
            avatarmedium: rs.avatarmedium,
            avatarfull: rs.avatarfull,
            status: rs.personastate.into(),
            comment_permission: rs.commentpermission.map(|p| match p {
                1 => true,
                _ => false,
            }),
            real_name: rs.realname,
            primaryclanid: rs.primaryclanid,
            time_created: rs.timecreated,
            country_code: rs.loccountrycode,
            city_id: rs.loccityid,
            game_id: rs.gameid,
            game_info: rs.gameextrainfo,
            gameserver_ip: rs.gameserverip,
        }
    }
}

/// Private Response type to simplify these utility types
type Response = ResponseWrapper<PlayersWrapper<RawSummary>>;

impl SteamClient {
    /// Gets vector of player/account [Summaries](Summary).
    ///
    /// Requires an API key and works with maximum of 100 [SteamIDs](SteamID).
    /// If the [SteamID] is invalid or user doesn't exist with the ID the API
    /// just drops the summary from the response. So don't assume the returned
    /// [Summaries](Summary) are in the same order as the [SteamIDs](SteamID).
    /// Always check the [SteamID] from the [Summary] struct.
    pub async fn get_player_summaries(&self, ids: Vec<SteamID>) -> Result<Vec<Summary>> {
        let api_key = self.api_key.as_ref().ok_or(Error {
            cause: "resolve_vanity_url requires an api key".to_owned(),
        })?;
        if ids.len() > 100 {
            return Err(Error {
                cause: "Too many IDs for get_player_summaries()".to_owned(),
            });
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

        Ok(resp.players.into_iter().map(|rs| rs.into()).collect())
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
