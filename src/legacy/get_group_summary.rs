use crate::error::Error;
use crate::steam_id::SteamID3;
use crate::utils::Result;
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::http::uri::{Scheme, Uri};

use serde::Deserialize;
use serde_xml_rs::from_str;

const AUTHORITY: &str = "steamcommunity.com";

#[derive(Deserialize, Debug)]
pub struct Group {
    #[serde(rename = "groupID64")]
    pub id: SteamID,
    #[serde(rename = "groupDetails")]
    pub details: GroupDetails,
    #[serde(rename = "memberCount")]
    pub member_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct GroupDetails {
    #[serde(rename = "groupName")]
    pub name: String,
    #[serde(rename = "groupURL")]
    pub url: String,
    #[serde(rename = "headline")]
    pub headline: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "avatarIcon")]
    pub avatar_icon: String,
    #[serde(rename = "avatarMedium")]
    pub avatar_medium: String,
    #[serde(rename = "avatarFull")]
    pub avatar_full: String,
    #[serde(rename = "membersInChat")]
    pub members_in_chat: u32,
    #[serde(rename = "membersInGame")]
    pub members_in_game: u32,
    #[serde(rename = "membersOnline")]
    pub members_online: u32,
}

impl SteamClient {
    /// Returns info about group by its id
    //
    // setting include_members to true will take a long time for a big group
    // and the return value will also be quite large. Thats why its false on default.
    pub async fn get_group_summary(
        &self,
        group_id: &SteamID,
        //include_members: Option<bool>,
    ) -> Result<Group> {
        let gid = SteamID3::from(*group_id).to_string();
        let path = format!(
            "/gid/{}/memberslistxml?xml=1",
            gid[5..gid.len() - 1].to_owned()
        );
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(AUTHORITY)
            .path_and_query(path)
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let stringified = String::from_utf8(to_bytes(raw_body).await?.to_vec())
            .map_err(|_| Error::client("unable to parse string from response"));
        let response: Group = from_str(&stringified?)?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn fetch_group_info() {
        let client = SteamClient::new();
        let group_id = SteamID::from(103582791456670032);
        let group_summary = block_on(client.get_group_summary(&group_id)).unwrap();
        println!("{group_summary:?}")
    }
}
