use std::str::FromStr;

use crate::client::SteamClient;
use crate::steam_id::SteamID;
use crate::utils::{concat_steam_ids, Error, PlayersWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetPlayerBans/v1";

#[derive(Debug, PartialEq)]
pub enum EconomyBanStatus {
    None,
    Probation,
    Unknown,
}

impl FromStr for EconomyBanStatus {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "none" => Ok(EconomyBanStatus::None),
            "probation" => Ok(EconomyBanStatus::Probation),
            _ => Ok(EconomyBanStatus::Unknown),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BanData {
    id: SteamID,
    community_banned: bool,
    vac_banned: bool,
    number_of_game_bans: u32,
    number_of_vac_bans: u32,
    days_since_last_ban: u32,
    economy_ban: EconomyBanStatus,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RawBanData {
    SteamId: String,
    CommunityBanned: bool,
    VACBanned: bool,
    NumberOfGameBans: u32,
    NumberOfVACBans: u32,
    DaysSinceLastBan: u32,
    EconomyBan: String,
}

impl From<RawBanData> for BanData {
    fn from(rbd: RawBanData) -> Self {
        BanData {
            id: rbd.SteamId.parse::<u64>().unwrap().into(),
            community_banned: rbd.CommunityBanned,
            vac_banned: rbd.VACBanned,
            number_of_game_bans: rbd.NumberOfGameBans,
            number_of_vac_bans: rbd.NumberOfVACBans,
            days_since_last_ban: rbd.DaysSinceLastBan,
            economy_ban: rbd.EconomyBan.parse::<EconomyBanStatus>().unwrap(),
        }
    }
}

/// Private Response type to simplify these utility types
type Response = PlayersWrapper<RawBanData>;

impl SteamClient {
    /// Gets vector of [BanData] structs
    ///
    /// If the [SteamID] is invalid or user doesn't exist with the ID
    /// the API just drops the [BanData] from the response. So don't assume
    /// the returned [BanDatas](BanData) are in the same order as the
    /// [SteamIDs](SteamID). Always check the [SteamID] from the [BanData]
    ///  struct
    pub async fn get_player_bans(&self, ids: Vec<SteamID>) -> Result<Vec<BanData>> {
        let id_query = concat_steam_ids(ids);
        let query = format!("key={}&steamids={}", self.api_key, id_query);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let response = self.client.get(uri).await;
        let body = response?.into_body();
        let players = serde_json::from_slice::<Response>(&to_bytes(body).await?)?.players;
        Ok(players.into_iter().map(|rbd| rbd.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn works() {
        let client = SteamClient::new(&env::var("STEAM_API_KEY").unwrap());
        let ban_data =
            tokio_test::block_on(client.get_player_bans(vec![SteamID::from(76561198061271782)]))
                .unwrap();
        assert_eq!(
            ban_data,
            vec![BanData {
                id: SteamID::from(76561198061271782),
                community_banned: false,
                vac_banned: false,
                number_of_game_bans: 0,
                number_of_vac_bans: 0,
                days_since_last_ban: 0,
                economy_ban: EconomyBanStatus::None,
            }]
        )
    }
}
