use crate::error::Error;
use serde::{de, de::Unexpected, Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use std::result::Result as StdResult;

pub const AUTHORITY: &str = "api.steampowered.com";

pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
pub struct ResponseMaybeEmpty<R> {
    #[serde(bound(deserialize = "R: Deserialize<'de>"))]
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub response: Option<R>,
}

#[derive(Deserialize)]
pub struct ResponseWrapper<R> {
    pub response: R,
}

#[derive(Deserialize)]
pub struct PlayersWrapper<P> {
    pub players: Vec<P>,
}

pub fn u64_from_str<'de, D>(deserializer: D) -> StdResult<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value
        .parse::<u64>()
        .map_err(|_| de::Error::invalid_value(Unexpected::Str(&value), &"u64 in a string"))
}
