use crate::client::SteamClient;
use crate::steam_id::SteamID;
use crate::utils::{Error, ResponseWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUser/ResolveVanityURL/v0001/";

pub enum URLType {
    Individual = 1,
    Group = 2,
    OfficalGameGroup = 3,
}

#[derive(Deserialize, Debug)]
struct Response {
    success: u8,
    steamid: Option<String>,
    message: Option<String>,
}

type Resp = ResponseWrapper<Response>;

impl SteamClient {
    /// Gets users [SteamID] based on users vanity url
    pub async fn resolve_vanity_url(
        &self,
        vanity_url: &str,
        url_type: Option<URLType>,
    ) -> Result<SteamID> {
        let type_query = match url_type {
            Some(url_type) => format!("&url_type={}", url_type as u8),
            None => "".to_owned(),
        };
        let query = format!(
            "key={}&vanityurl={}{}",
            self.api_key, vanity_url, type_query
        );
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let response = self.client.get(uri).await;
        let body = response?.into_body();
        let resp = serde_json::from_slice::<Resp>(&to_bytes(body).await?)?.response;

        if resp.success == 1 {
            if let Some(id) = resp.steamid {
                Ok(id.parse::<u64>()?.into())
            } else {
                Err(Error::new(
                    "response had success flag but didn't contain a steamid".to_owned(),
                ))
            }
        } else {
            Err(Error::new(format!(
                "request failed, {}",
                resp.message.unwrap_or("no message".to_owned())
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio_test::assert_err;

    use super::*;
    use std::env;

    #[test]
    fn resolve_correct_url() {
        let client = SteamClient::new(&env::var("STEAM_API_KEY").unwrap());
        let id = tokio_test::block_on(client.resolve_vanity_url("petesammakko", None)).unwrap();
        assert_eq!(id, SteamID::from(76561198061271782));
    }

    #[test]
    fn handle_incorrect_url() {
        let client = SteamClient::new(&env::var("STEAM_API_KEY").unwrap());
        let id = tokio_test::block_on(client.resolve_vanity_url("", None));
        assert_err!(id, "invalid ID should result in error");
    }
}
