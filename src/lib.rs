//! Easy to use API wrapper for Steam web API.
//!
//! Provides an easy way to use Steam web API asynchronously. The client is lightweight,
//! since it's build on top of [hyper]
//!
//! [hyper]: https://hyper.rs/

mod client;
mod steam_id;
pub mod steam_user;
pub mod steam_user_stats;
mod utils;

pub use client::SteamClient;
pub use steam_id::SteamID;
