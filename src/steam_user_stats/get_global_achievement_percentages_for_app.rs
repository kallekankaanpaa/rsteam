use std::num::NonZeroU64;

use crate::client::SteamClient;
use crate::error::Error;
use crate::utils::{Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/";

#[derive(Deserialize, Debug)]
pub struct AchievementData {
    /// Name of the achievement as unlocalized token
    pub name: String,
    // Percentage of players who have unlocked the achievement
    pub percent: f32,
}

#[derive(Deserialize)]
struct Achievements {
    achievements: Vec<AchievementData>,
}

#[derive(Deserialize)]
struct Response {
    achievementpercentages: Achievements,
}

impl SteamClient {
    /// Fetches a vector of [AchievementData] structs for the given game_id.
    ///
    /// Works without an API key.
    pub async fn get_global_achievement_percentages_for_app(
        &self,
        game_id: NonZeroU64,
    ) -> Result<Vec<AchievementData>> {
        let query = format!("gameid={}", game_id);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response =
            serde_json::from_slice(&to_bytes(raw_body).await?).map_err(|_| {
                Error::Client(
                    "No game with game_id or developer hasn't enabled achievements".to_owned(),
                )
            })?;

        Ok(response.achievementpercentages.achievements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::{assert_err, block_on};

    #[test]
    fn correct_csgo_achievements() {
        let client = SteamClient::new();
        let game_id = NonZeroU64::new(730).unwrap();
        let achievements =
            block_on(client.get_global_achievement_percentages_for_app(game_id)).unwrap();

        assert_eq!(1, achievements.len());
    }

    #[test]
    fn unknown_game_id_handeled_correctly() {
        let client = SteamClient::new();
        let game_id = NonZeroU64::new(731).unwrap();
        let achievements = block_on(client.get_global_achievement_percentages_for_app(game_id));

        assert_err!(achievements);
    }
}
