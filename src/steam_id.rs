use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const INDIVIDUAL_IDENTIFIER: u64 = 0x0110000100000000_u64;
const GROUP_IDENTIFIER: u64 = 0x0170000000000000_u64;

#[derive(Debug)]
pub enum SteamIDError {
    ParseError,
    ConversionError,
}

impl From<ParseIntError> for SteamIDError {
    fn from(_: ParseIntError) -> Self {
        SteamIDError::ParseError
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Universe {
    Individual = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4,
    RC = 5,
}

impl TryFrom<u8> for Universe {
    type Error = SteamIDError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Universe::Individual),
            1 => Ok(Universe::Public),
            2 => Ok(Universe::Beta),
            3 => Ok(Universe::Internal),
            4 => Ok(Universe::Dev),
            5 => Ok(Universe::RC),
            _ => Err(SteamIDError::ParseError),
        }
    }
}

impl FromStr for Universe {
    type Err = SteamIDError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>()?.try_into()
    }
}

/* SteamID can only be used to represent individual(user) steamids */
#[derive(PartialEq, Debug)]
pub struct SteamID2 {
    pub universe: Universe,
    pub id_number: u8,
    pub account_number: u32,
}

impl FromStr for SteamID2 {
    type Err = SteamIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let universe = s[6..7].parse::<Universe>()?;
        let id_number = s[8..9].parse::<u8>()?;
        let account_number = s[10..].parse::<u32>()?;

        Ok(SteamID2 {
            universe,
            id_number,
            account_number,
        })
    }
}

impl fmt::Display for SteamID2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "STEAM_{}:{}:{}",
            self.universe as u8, self.id_number, self.account_number
        )
    }
}

impl TryFrom<SteamID> for SteamID2 {
    type Error = SteamIDError;
    fn try_from(id: SteamID) -> Result<Self, Self::Error> {
        let id_type = (id.0 << 8) >> 52;
        if id_type == 1 {
            let x = (id.0 >> 56) as u8;
            let y = id.0 & 1;
            let z = (id.0 - y - INDIVIDUAL_IDENTIFIER) / 2;
            Ok(SteamID2 {
                universe: x.try_into()?,
                id_number: y as u8,
                account_number: z as u32,
            })
        } else {
            Err(SteamIDError::ConversionError)
        }
    }
}

pub struct SteamID(u64);

impl From<SteamID2> for SteamID {
    fn from(id: SteamID2) -> Self {
        SteamID(id.account_number as u64 * 2 + INDIVIDUAL_IDENTIFIER + id.id_number as u64)
    }
}

pub enum IDType {
    Invalid,
    Individual,
    Multiseat,
    GameServer,
    AnonGameServer,
    Pending,
    ContentServer,
    Clan,
    Chat,
    AnonUser,
}

pub struct SteamID3 {
    pub id_type: IDType,
    pub universe: Universe,
    pub account_id: u32,
}

mod tests {
    use super::*;

    #[test]
    fn steam_id() {
        let steam_id = SteamID2 {
            universe: Universe::Individual,
            id_number: 0,
            account_number: 50503027,
        };
        assert_eq!("STEAM_0:0:50503027".parse::<SteamID2>().unwrap(), steam_id);

        assert_eq!(format!("{}", steam_id), "STEAM_0:0:50503027");

        /*
        assert_eq!(
            "[U:1:101006054]".parse::<SteamID>().unwrap(),
            SteamID(76561198061271782)
        );
        group id = 103582791469184191
        user id =  76561198061271782
        */
    }
}
