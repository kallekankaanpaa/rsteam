use std::env;

use rsteam::steam_user::{BanData, URLType};
use rsteam::{SteamClient, SteamID};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Ok(api_key) = env::var("STEAM_API_KEY") else {
        println!("Remember to set the STEAM_API_KEY environment variable");
        return Ok(());
    };

    let Some(vanity_url) = env::args().nth(1) else {
        println!("Please provide vanity url (custom steam id).");
        return Ok(());
    };

    let client = SteamClient::with_api_key(&api_key);

    let id = client
        .resolve_vanity_url(&vanity_url, Some(URLType::Individual))
        .await?;

    let friends = client.get_friend_list(&id, None).await?;

    let ids: Vec<SteamID> = friends.into_iter().map(|friend| friend.id).collect();

    let banned_friends: Vec<BanData> = client
        .get_player_bans(&ids)
        .await?
        .into_iter()
        .filter(|ban| ban.community_banned || ban.vac_banned)
        .collect();

    let summaries = client.get_player_summaries(&banned_friends.iter().map(|b| b.id).collect::<Vec<SteamID>>()).await?;

    for (ban, summary) in banned_friends.iter().zip(summaries.iter()) {
        println!(
            "{}, game bans: {}, vac bans: {}, since last: {}",
            summary.profile_name, ban.number_of_game_bans, ban.number_of_vac_bans, ban.days_since_last_ban
        );
    }

    Ok(())
}
