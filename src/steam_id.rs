use std::convert::TryFrom;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct SteamError;

impl From<ParseIntError> for SteamError {
    fn from(_: ParseIntError) -> Self {
        SteamError
    }
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq)]
pub struct SteamID {
    universe: u8,
    account_type: u8,
    account_id: u32,
}

impl fmt::Display for SteamID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id64: u64 = self.into();
        write!(f, "{}", id64)
    }
}

impl Into<u64> for &SteamID {
    fn into(self) -> u64 {
        let universe = (self.universe as u64) << 56;
        let account_type = (self.account_type as u64) << 52;
        let instance = 1_u64 << 32;
        let account_id = self.account_id as u64;
        0_u64 | universe | account_type | instance | account_id
    }
}

impl From<u64> for SteamID {
    fn from(value: u64) -> Self {
        let universe = (value >> 56) as u8;
        let account_type = (value << 8 >> 60) as u8;
        let account_id = value as u32;
        SteamID {
            universe,
            account_type,
            account_id,
        }
    }
}

impl From<SteamID2> for SteamID {
    fn from(id: SteamID2) -> Self {
        id.0
    }
}

impl From<SteamID3> for SteamID {
    fn from(id: SteamID3) -> Self {
        id.0
    }
}

#[derive(PartialEq, Debug)]
struct SteamID2(SteamID);

impl fmt::Display for SteamID2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let SteamID {
            account_id,
            universe,
            ..
        } = self.0;
        write!(
            f,
            "STEAM_{}:{}:{}",
            universe,
            account_id & 1,
            account_id / 2
        )
    }
}

impl TryFrom<SteamID> for SteamID2 {
    type Error = SteamError;

    fn try_from(value: SteamID) -> Result<Self, Self::Error> {
        if value.account_type == 1 {
            Ok(SteamID2(value))
        } else {
            Err(SteamError)
        }
    }
}

impl FromStr for SteamID2 {
    type Err = SteamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let universe: u8 = s[6..7].parse()?;
        let y: u32 = s[8..9].parse()?;
        let z: u32 = s[10..].parse()?;
        let account_id = 2 * z + y;
        Ok(SteamID2(SteamID {
            universe: if universe == 0 { 1 } else { universe },
            account_type: 1, // SteamID2 can only represent individual accounts
            account_id,
        }))
    }
}

#[derive(PartialEq, Debug)]
struct SteamID3(SteamID);

impl fmt::Display for SteamID3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let SteamID {
            account_id,
            account_type,
            universe,
        } = self.0;
        let account_type_letter = match account_type {
            0 => Some('I'),
            1 => Some('U'),
            2 => Some('M'),
            3 => Some('G'),
            4 => Some('A'),
            5 => Some('P'),
            6 => Some('C'),
            7 => Some('g'),
            8 => Some('T'),
            10 => Some('a'),
            _ => None,
        };
        if let Some(letter) = account_type_letter {
            write!(f, "[{}:{}:{}]", letter, universe, account_id)
        } else {
            write!(f, "Account can not be represented as 32-bit id")
        }
    }
}

impl From<SteamID> for SteamID3 {
    fn from(id: SteamID) -> Self {
        SteamID3(id)
    }
}

impl FromStr for SteamID3 {
    type Err = SteamError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let account_type: u8 = match &s[1..2] {
            "I" => Ok(0),
            "U" => Ok(1),
            "M" => Ok(2),
            "G" => Ok(3),
            "A" => Ok(4),
            "P" => Ok(5),
            "C" => Ok(6),
            "g" => Ok(7),
            "T" | "L" | "c" => Ok(8),
            "a" => Ok(10),
            _ => Err(SteamError),
        }?;
        let universe: u8 = s[3..4].parse()?;
        let account_id: u32 = s[5..s.len() - 1].parse()?;
        Ok(SteamID3(SteamID {
            universe,
            account_type,
            account_id,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::steam_id::*;
    const CORRECT_ID: SteamID = SteamID {
        universe: 1,
        account_type: 1,
        account_id: 101006054,
    };

    #[test]
    fn id64() {
        let id64: u64 = (&CORRECT_ID).into();
        assert_eq!(SteamID::from(76561198061271782), CORRECT_ID);
        assert_eq!(id64, 76561198061271782)
    }

    #[test]
    fn id2() {
        let id2 = SteamID2(CORRECT_ID);
        assert_eq!("STEAM_0:0:50503027".parse::<SteamID2>().unwrap(), id2);
        assert_eq!("STEAM_1:0:50503027".parse::<SteamID2>().unwrap(), id2);
        assert_eq!(format!("{}", id2), "STEAM_1:0:50503027");
    }

    #[test]
    fn id3() {
        let id3 = SteamID3(CORRECT_ID);
        assert_eq!("[U:1:101006054]".parse::<SteamID3>().unwrap(), id3);
        assert_eq!(format!("{}", id3), "[U:1:101006054]");
    }
}
