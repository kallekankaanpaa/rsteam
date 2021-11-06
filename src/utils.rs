use crate::SteamID;
use serde::Deserialize;
use std::fmt;

pub const AUTHORITY: &str = "api.steampowered.com";

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    APIKeyRequired,
    Other { cause: String },
    NoAPIKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::APIKeyRequired => write!(f, "The api requires an API key"),
            ErrorKind::Other { cause } => write!(f, "{}", cause),
            ErrorKind::NoAPIKey => write!(f, "The client has no api key"),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl From<hyper::http::Error> for Error {
    fn from(original: hyper::http::Error) -> Self {
        Error {
            kind: ErrorKind::Other {
                cause: original.to_string(),
            },
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(original: hyper::Error) -> Self {
        Error {
            kind: ErrorKind::Other {
                cause: original.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(original: serde_json::Error) -> Self {
        Error {
            kind: ErrorKind::Other {
                cause: original.to_string(),
            },
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(original: std::num::ParseIntError) -> Self {
        Error {
            kind: ErrorKind::Other {
                cause: original.to_string(),
            },
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

pub(crate) fn format_query_param<T: std::fmt::Display>(
    optional_param: Option<T>,
    param_name: &str,
) -> String {
    optional_param
        .map(|p| format!("&{}={}", param_name, p))
        .unwrap_or("".to_owned())
}
