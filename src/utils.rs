use crate::error::Error;
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
