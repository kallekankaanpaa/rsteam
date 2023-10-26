use crate::client::SteamClient;
use crate::error::Error;
use crate::steam_id::SteamID;
use crate::utils::{ResponseWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;
use std::fmt;

const PATH: &str = "/ISteamUser/ResolveVanityURL/v0001/";

#[derive(Clone, Copy)]
pub enum URLType {
    Individual = 1,
    Group = 2,
    OfficalGameGroup = 3,
}

impl fmt::Display for URLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    success: u8,
    steamid: Option<String>,
    message: Option<String>,
}

type Resp = ResponseWrapper<Response>;

impl SteamClient {
    /// Gets users [`SteamID`] based on users vanity url
    ///
    /// Requires an API key. Default [`URLType`] is individual.
    pub async fn resolve_vanity_url(
        &self,
        vanity_url: &str,
        url_type: Option<URLType>,
    ) -> Result<SteamID> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| Error::client("API key required"))?;

        let type_query = optional_query!(url_type);
        let query = format!("key={api_key}&vanityurl={vanity_url}{type_query}");
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{PATH}?{query}"))
            .build()?;

        let raw_response = self.client.get(uri).await;
        let raw_body = raw_response?.into_body();
        let response = serde_json::from_slice::<Resp>(&to_bytes(raw_body).await?)?.response;

        let Response {
            success,
            steamid,
            message,
        } = response;

        if success == 1 {
            if let Some(id) = steamid {
                Ok(id
                    .parse::<u64>()
                    .map_err(|_| Error::Client("returned steamid is invalid".to_owned()))?
                    .into())
            } else {
                Err(Error::Client(
                    "request was successfull but didn't contain steamid".to_owned(),
                ))
            }
        } else {
            Err(Error::Client(format!(
                "request failed: {}",
                message.unwrap_or_else(|| "no message".to_owned())
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
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = tokio_test::block_on(client.resolve_vanity_url("petesammakko", None)).unwrap();
        assert_eq!(id, SteamID::from(76561198061271782));
    }

    #[test]
    fn handle_incorrect_url() {
        let client = SteamClient::with_api_key(&env::var("STEAM_API_KEY").unwrap());
        let id = tokio_test::block_on(client.resolve_vanity_url("", None));
        assert_err!(id, "invalid ID should result in error");
    }
}
