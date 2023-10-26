use crate::client::SteamClient;
use crate::error::Error;
use crate::steam_id::SteamID;
use crate::utils::{PlayersWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/GetPlayerBans/v1";

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EconomyBanStatus {
    None,
    Probation,
    Unknown,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct BanData {
    #[serde(rename = "SteamId")]
    pub id: SteamID,
    #[serde(rename = "CommunityBanned")]
    pub community_banned: bool,
    #[serde(rename = "VACBanned")]
    pub vac_banned: bool,
    #[serde(rename = "NumberOfGameBans")]
    pub number_of_game_bans: u32,
    #[serde(rename = "NumberOfVACBans")]
    pub number_of_vac_bans: u32,
    #[serde(rename = "DaysSinceLastBan")]
    pub days_since_last_ban: u32,
    #[serde(rename = "EconomyBan")]
    pub economy_ban: EconomyBanStatus,
}

/// Private Response type to simplify these utility types
type Response = PlayersWrapper<BanData>;

impl SteamClient {
    /// Gets vector of [BanData] structs
    ///
    /// Requires an API key. If the [SteamID] is invalid or user doesn't exist
    /// with the ID the API just drops the [BanData] from the response. So
    /// don't assume the returned [BanDatas](BanData) are in the same order as
    /// the [SteamIDs](SteamID). Always check the [SteamID] from the [BanData]
    /// struct
    pub async fn get_player_bans(&self, ids: &[SteamID]) -> Result<Vec<BanData>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

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
        let players = serde_json::from_slice::<Response>(&to_bytes(body).await?)?.players;
        Ok(players)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn works() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let ban_data =
            tokio_test::block_on(client.get_player_bans(&vec![SteamID::from(76561198061271782)]))
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
