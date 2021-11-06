use std::num::NonZeroU32;

use crate::client::SteamClient;
use crate::utils::{Error, ErrorKind, ResponseWrapper, Result, AUTHORITY};
use hyper::body::to_bytes;
use hyper::Uri;
use serde::Deserialize;

const PATH: &str = "/ISteamUserStats/GetNumberOfCurrentPlayers/v1/";

#[derive(Deserialize)]
struct PlayerCount {
    player_count: Option<u32>,
    result: u32,
}

type Response = ResponseWrapper<PlayerCount>;

impl SteamClient {
    /// Returns the player count of the app/game
    ///
    /// Works without an API key.
    pub async fn get_number_of_current_players(&self, game_id: NonZeroU32) -> Result<u32> {
        let query = format!("appid={}", game_id);
        let uri = Uri::builder()
            .scheme("https")
            .authority(AUTHORITY)
            .path_and_query(format!("{}?{}", PATH, query))
            .build()?;

        let raw_response = self.client.get(uri).await?;
        let raw_body = raw_response.into_body();
        let response: Response =
            serde_json::from_slice(&to_bytes(raw_body).await?).map_err(|_| {
                Error::new(ErrorKind::Other {
                    cause: "No game with game_id".to_owned(),
                })
            })?;

        let PlayerCount {
            player_count,
            result,
        } = response.response;

        if result == 1 && player_count.is_some() {
            Ok(player_count.unwrap())
        } else {
            Err(Error::new(ErrorKind::Other {
                cause: "Request failed check that game_id is valid".to_owned(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::{assert_err, block_on};

    #[test]
    fn correct_csgo_achievements() {
        let client = SteamClient::new();
        let game_id = NonZeroU32::new(730).unwrap();
        let player_count = block_on(client.get_number_of_current_players(game_id)).unwrap();

        assert!(player_count > 100000);
    }

    #[test]
    fn unknown_game_id_handeled_correctly() {
        let client = SteamClient::new();
        let game_id = NonZeroU32::new(731).unwrap();
        let result = block_on(client.get_number_of_current_players(game_id));

        assert_err!(result);
    }
}
