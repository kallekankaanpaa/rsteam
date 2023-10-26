use std::env;

use rsteam::{steam_id::SteamID3, SteamID};

fn main() {
    let id64 = match env::args().nth(1) {
        Some(id) => id.parse::<u64>().unwrap(),
        None => {
            panic!("please provide 64-bit steamid");
        }
    };

    let id = SteamID::from(id64);
    let id3 = SteamID3::from(id);

    println!("{id}");
    println!("{id3}");
}
