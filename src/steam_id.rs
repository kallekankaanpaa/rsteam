use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

const v: u64 = 0x0110000100000000_u64;

#[derive(PartialEq, Debug)]
pub struct SteamID(u64);

impl FromStr for SteamID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^STEAM_\d:(\d):(\d{8})$").unwrap();
            static ref ID32_RE: Regex = Regex::new(r"\[[IUMGAPCgTLca]:\d:(\d{9})\]$").unwrap();
        }

        if let Some(captures) = ID_RE.captures(s) {
            let y = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let z = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Ok(SteamID(z * 2 + v + y))
        } else if let Some(captures) = ID32_RE.captures(s) {
            let account_id = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let y = account_id & 1;
            let z = account_id / 2_u64;
            Ok(SteamID(z * 2 + v + y))
        } else {
            s.parse::<u64>().map(|id| SteamID(id))
        }
    }
}

impl SteamID {
    pub fn new(id: u64) -> Self {
        SteamID(id)
    }

    pub fn id64(&self) -> u64 {
        self.0
    }

    pub fn id32(&self) -> String {
        let account_id = self.0 - v;
        let y = self.0 & 1;
        format!("[{}:{}:{}]", "U", y, account_id)
    }

    pub fn raw(&self) -> String {
        let y = self.0 & 1;
        let z = (self.0 - v - y) / 2;
        let x = 1_u64; // where from?
        format!("STEAM_{}:{}:{}", x, y, z)
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_steam_id() {
        assert_eq!(
            "STEAM_0:0:50503027".parse::<SteamID>().unwrap(),
            SteamID(76561198061271782)
        );

        assert_eq!(
            "[U:1:101006054]".parse::<SteamID>().unwrap(),
            SteamID(76561198061271782)
        );
    }
}
