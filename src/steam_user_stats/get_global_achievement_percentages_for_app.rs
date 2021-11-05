use crate::client::SteamClient;
use crate::utils::{Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUserStats/GetGlobalAchievementPercentagesForApp/v0002/";

#[derive(Deserialize, Debug)]
pub struct AchievementData {
    name: String,
    percent: f32,
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
    pub async fn get_global_achievement_percentages_for_app(
        &self,
        game_id: u32,
    ) -> Result<Vec<AchievementData>> {
        let query = format!("key={}&gameid={}", self.api_key, game_id);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response = serde_json::from_slice(&to_bytes(raw_body).await?)?;

        Ok(response.achievementpercentages.achievements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn correct_csgo_achievements() {
        let client = SteamClient::new(&env::var("STEAM_API_KEY").unwrap());
        let achievements =
            tokio_test::block_on(client.get_global_achievement_percentages_for_app(730)).unwrap();

        assert_eq!(167, achievements.len());
    }
}
