//! Implementations for the `IPlayerService` interface

mod get_badges;
mod get_community_badge_progress;
mod get_owned_games;
mod get_recently_played_games;
mod get_steam_level;
mod is_playing_shared_game;

pub use get_badges::{Badge, Badges};
pub use get_community_badge_progress::Quest;
pub use get_owned_games::{Game as OwnedGame, OwnedGames};
pub use get_recently_played_games::{Game as RecentlyPlayedGame, RecentlyPlayedGames};
