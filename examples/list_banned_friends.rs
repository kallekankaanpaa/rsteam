use std::env;

use rsteam::steam_user::BanData;
use rsteam::{SteamClient, SteamID};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("STEAM_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Remember to set the STEAM_API_KEY environment variable");
            return Ok(());
        }
    };

    let vanity_url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Please provide vanity url (custom steam id).");
            return Ok(());
        }
    };

    let client = SteamClient::new(&api_key);

    let id = client.resolve_vanity_url(&vanity_url).await?;

    let friends = client.get_friend_list(id, None).await?;

    let ids: Vec<SteamID> = friends.into_iter().map(|friend| friend.id).collect();

    let banned_friends: Vec<BanData> = client
        .get_player_bans(ids)
        .await?
        .into_iter()
        .filter(|ban| ban.community_banned || ban.vac_banned)
        .collect();

    for ban in banned_friends {
        println!(
            "{}, game bans: {}, vac bans: {}, since last: {}",
            ban.id, ban.number_of_game_bans, ban.number_of_vac_bans, ban.days_since_last_ban
        );
    }

    Ok(())
}
