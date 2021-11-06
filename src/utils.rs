use crate::error::Error;
use crate::SteamID;
use serde::Deserialize;

pub const AUTHORITY: &str = "api.steampowered.com";

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
