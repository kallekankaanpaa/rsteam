use std::result::Result as StdResult;
use crate::error::Error;
use serde::{de, de::Unexpected, Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_default_from_empty_object;

pub const AUTHORITY: &str = "api.steampowered.com";

pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
pub(crate) struct ResponseMaybeEmpty<R> {
    #[serde(bound(deserialize = "R: Deserialize<'de>"))]
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub(crate) response: Option<R>
}

#[derive(Deserialize)]
pub(crate) struct ResponseWrapper<R> {
    pub(crate) response: R,
}

#[derive(Deserialize)]
pub(crate) struct PlayersWrapper<P> {
    pub(crate) players: Vec<P>,
}

pub(crate) fn u64_from_str<'de, D>(deserializer: D) -> StdResult<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    match value.parse::<u64>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(de::Error::invalid_value(
            Unexpected::Str(&value),
            &"u64 in a string",
        )),
    }
}
