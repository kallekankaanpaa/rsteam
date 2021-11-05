use std::env;

use rsteam::SteamClient;

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

    println!("{}", id);

    Ok(())
}
