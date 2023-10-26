use std::cmp;
use std::env;
use std::time;

use rsteam::steam_id::{SteamID2, SteamID3};
use rsteam::steam_user::{BanData, Status};
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

    let client = SteamClient::with_api_key(&api_key);

    let id = client.resolve_vanity_url(&vanity_url, None).await?;
    let id_vec = vec![id];

    let ban_datas = client.get_player_bans(&id_vec).await?;
    let ban_data = ban_datas.first().ok_or("Fetching ban data failed")?;
    let summaries = client.get_player_summaries(&id_vec).await?;
    let summary = summaries.first().ok_or("Fetching summary failed")?;
    let group_list = client.get_user_group_list(&id).await?;
    let friend_list = client.get_friend_list(&id, None).await?;

    let badges = client.get_badges(&id).await?;
    let owned_games = client
        .get_owned_games(&id, Some(true), None, None, None)
        .await?;
    let three_recent_games = client.get_recently_played_games(&id, Some(3)).await?;
    let steam_level = client.get_steam_level(&id).await?;

    println!("Steam profile data for {}", summary.profile_name);
    println!(
        "SteamID: {} / {} / {}",
        &id,
        SteamID3::from(id),
        SteamID2::try_from(id)
            .map(|id| id.to_string())
            .unwrap_or("ID can't be represented as a legacy ID".to_owned())
    );
    let community_banned = match ban_data.community_banned {
        true => "\x1b[91mcommunity banned\x1b[0m",
        false => "\x1b[92mnot community banned\x1b[0m",
    };

    let vac_banned = match ban_data.vac_banned {
        true => "\x1b[91mVAC banned\x1b[0m",
        false => "\x1b[92mnot VAC banned\x1b[0m",
    };

    println!("Ban status: {community_banned}, {vac_banned}");
    println!("Steam level: {steam_level}");
    println!(
        "Owns {} games and has {} badges",
        owned_games.game_count,
        badges.badges.len()
    );
    println!(
        "User belongs to {} groups and their primary group is {}",
        group_list.len(),
        match &summary.primary_clan_id {
            Some(id) => client.get_group_summary(id).await?.details.name,
            None => "User has no primary group".to_owned(),
        }
    );
    let friend_ids: Vec<SteamID> = friend_list.iter().map(|f| f.id).collect();
    let banned_friends: Vec<BanData> = client
        .get_player_bans(&friend_ids)
        .await?
        .into_iter()
        .filter(|b| b.community_banned || b.vac_banned)
        .collect();
    println!(
        "User has {} friends, {:.1}% of who are banned",
        friend_list.len(),
        banned_friends.len() as f32 / cmp::max(friend_list.len(), 1) as f32 * 100_f32
    );

    if let Some(three_recent_games) = three_recent_games {
        let recent_3: Vec<String> = three_recent_games
            .games
            .iter()
            .map(|g| format!("{} ({:.1}h)", g.name, g.playtime_2weeks as f32 / 60_f32))
            .collect();
        println!(
            "The user has played, {} and {} other games recently",
            recent_3.join(", "),
            cmp::max(three_recent_games.total_count, 3) - 3,
        );
    }

    match (&summary.status, summary.last_logoff) {
        (Status::Offline, Some(last_logoff)) => println!(
            "User was last online {} days ago",
            (time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Something funny happened").as_secs() - last_logoff as u64 / 60 / 60 / 24)
        ),
        _ => println!("User is currently {:?}", summary.status),
    }

    match summary.time_created {
        Some(secs) => {
            let created_secs = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Something funny happened").as_secs() - secs as u64;
            println!("Account created {} days ago", (created_secs / 60 / 60 / 24))
        }
        None => println!("Unknown account age"),
    }
    println!("Profile visibility: {:?}", summary.visibility);

    Ok(())
}
