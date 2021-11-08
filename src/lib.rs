//! Easy to use API wrapper for Steam web API.
//!
//! Provides an easy way to use Steam web API asynchronously. The client is lightweight,
//! since it's build on top of [hyper]
//!
//! [hyper]: https://hyper.rs/

mod client;
pub mod error;
#[macro_use]
mod macros;
pub mod player_service;
pub mod steam_apps;
mod steam_id;
pub mod steam_news;
pub mod steam_user;
pub mod steam_user_stats;
mod utils;

pub use client::SteamClient;
pub use steam_id::SteamID;
