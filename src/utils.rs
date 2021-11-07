use crate::error::Error;
use crate::SteamID;
use serde::{de, de::Unexpected, Deserialize, Deserializer};
use std::result::Result as StdResult;

pub const AUTHORITY: &str = "api.steampowered.com";

pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
pub(crate) struct ResponseWrapper<R> {
    pub(crate) response: R,
}

#[derive(Deserialize)]
pub(crate) struct PlayersWrapper<P> {
    pub(crate) players: Vec<P>,
}

pub(crate) fn concat_steam_ids(ids: Vec<SteamID>) -> String {
    ids.iter()
        .map(|id| id.to_string())
        .fold("".to_owned(), |a, b| a + &b + ",")
}

pub(crate) fn format_query_param<T: std::fmt::Display>(
    optional_param: Option<T>,
    param_name: &str,
) -> String {
    optional_param
        .map(|p| format!("&{}={}", param_name, p))
        .unwrap_or_else(String::new)
}

pub(crate) fn bool_from_int_maybe_missing<'de, D>(
    deserializer: D,
) -> StdResult<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match u32::deserialize(deserializer) {
        Ok(integer) => match integer {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(other as u64),
                &"zero or one",
            )),
        },
        Err(_) => Ok(None),
    }
}
