use crate::SteamID;
use serde::Deserialize;
use std::fmt;

pub const AUTHORITY: &str = "api.steampowered.com";

#[derive(Debug)]
pub struct Error {
    pub cause: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(cause: String) -> Self {
        Error { cause }
    }
}

impl From<hyper::http::Error> for Error {
    fn from(original: hyper::http::Error) -> Self {
        Error {
            cause: original.to_string(),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(original: hyper::Error) -> Self {
        Error {
            cause: original.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(original: serde_json::Error) -> Self {
        Error {
            cause: original.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(original: std::num::ParseIntError) -> Self {
        Error {
            cause: original.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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
