use crate::error::Error;
use crate::steam_id::SteamID3;
use crate::utils::Result;
use crate::{SteamClient, SteamID};

use hyper::body::to_bytes;
use hyper::http::uri::{Scheme, Uri};

use serde::Deserialize;
use serde_xml_rs::from_str;

use futures::future::try_join_all;

const AUTHORITY: &str = "steamcommunity.com";

#[derive(Deserialize, Debug)]
struct Members {
    #[serde(rename = "steamID64")]
    steam_ids: Vec<SteamID>,
}

#[derive(Deserialize, Debug)]
struct Group {
    #[serde(rename = "nextPageLink")]
    _next_page_url: Option<String>,
    #[serde(rename = "memberCount")]
    member_count: u32,
    #[serde(rename = "totalPages")]
    total_page_amount: u32,
    members: Members,
}

#[derive(Deserialize, Debug)]
struct GroupDetails {}

impl SteamClient {
    /// Returns info about group by its id
    ///
    /// The running time of the function is linear to the size of the group.
    /// This is because the API returns just 1000 members per request so for
    /// a group with 2 million members it needs to make 2000 requests to the API.
    /// Thats why caching the result is recommended instead of calling the function
    /// multiple times for same group.
    pub async fn list_group_members(&self, group_id: &SteamID) -> Result<Vec<SteamID>> {
        let gid = SteamID3::from(*group_id).to_string();
        let legacy_id = gid[5..gid.len() - 1].to_owned();

        let path = format!("/gid/{legacy_id}/memberslistxml?xml=1");
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(AUTHORITY)
            .path_and_query(path)
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let stringified = String::from_utf8(to_bytes(raw_body).await?.to_vec())
            .map_err(|_| Error::client("unable to parse string from response"));
        let mut first_page: Group = from_str(&stringified?)?;

        // First allocate space for all members and then add members from first page
        let mut members: Vec<SteamID> =
            Vec::with_capacity(first_page.member_count.try_into().unwrap());
        members.append(&mut first_page.members.steam_ids);

        let mut futures = vec![self.fetch_page(group_id, 2)];
        for page_number in 3..first_page.total_page_amount + 1 {
            futures.push(self.fetch_page(group_id, page_number))
        }

        for mut page in try_join_all(futures).await? {
            members.append(&mut page);
        }
        Ok(members)
    }

    async fn fetch_page(&self, group_id: &SteamID, page: u32) -> Result<Vec<SteamID>> {
        let gid = SteamID3::from(*group_id).to_string();
        let legacy_id = gid[5..gid.len() - 1].to_owned();

        let path = format!("/gid/{legacy_id}/memberslistxml?xml=1&p={page}");
        let uri = Uri::builder()
            .scheme(Scheme::HTTPS)
            .authority(AUTHORITY)
            .path_and_query(path)
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let stringified = String::from_utf8(to_bytes(raw_body).await?.to_vec())
            .map_err(|_| Error::client("unable to parse string from response"));
        let group: Group = from_str(&stringified?)?;

        Ok(group.members.steam_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn fetch_1_8m_members() {
        let client = SteamClient::new();
        let group_id = SteamID::from(103582791456670032);
        let group = block_on(client.get_group_summary(&group_id)).unwrap();
        let members = block_on(client.list_group_members(&group_id)).unwrap();
        assert_eq!(members.len(), group.member_count as usize);
    }

    #[test]
    fn fetch_12k_members() {
        let client = SteamClient::new();
        let group_id = SteamID::from(103582791463067899);
        let group = block_on(client.get_group_summary(&group_id)).unwrap();
        let members = block_on(client.list_group_members(&group_id)).unwrap();
        assert_eq!(members.len(), group.member_count as usize);
    }
}
