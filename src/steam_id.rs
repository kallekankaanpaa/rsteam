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
    P2PSuperSeeder,
    AnonUser,
}

impl FromStr for IDType {
    type Err = SteamIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "I" => Ok(IDType::Invalid),
            "U" => Ok(IDType::Individual),
            "M" => Ok(IDType::Multiseat),
            "G" => Ok(IDType::GameServer),
            "A" => Ok(IDType::AnonGameServer),
            "P" => Ok(IDType::Pending),
            "C" => Ok(IDType::ContentServer),
            "g" => Ok(IDType::Clan),
            "T" | "L" | "c" => Ok(IDType::Chat),
            "a" => Ok(IDType::AnonUser),
            _ => Err(SteamIDError::ParseError),
        }
    }
}

impl TryFrom<u8> for IDType {
    type Error = SteamIDError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IDType::Invalid),
            1 => Ok(IDType::Individual),
            2 => Ok(IDType::Multiseat),
            3 => Ok(IDType::GameServer),
            4 => Ok(IDType::AnonGameServer),
            5 => Ok(IDType::Pending),
            6 => Ok(IDType::ContentServer),
            7 => Ok(IDType::Clan),
            8 => Ok(IDType::Chat),
            9 => Ok(IDType::P2PSuperSeeder),
            10 => Ok(IDType::AnonUser),
            _ => Err(SteamIDError::ParseError),
        }
    }
}

impl fmt::Display for IDType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letter = match self {
            IDType::Invalid => 'I',
            IDType::Individual => 'U',
            IDType::Multiseat => 'M',
            IDType::GameServer => 'G',
            IDType::AnonGameServer => 'A',
            IDType::Pending => 'P',
            IDType::ContentServer => 'C',
            IDType::Clan => 'g',
            IDType::Chat => 'T', // How about 'l' or 'c'
            IDType::AnonUser => 'a',
            IDType::P2PSuperSeeder => panic!("P2PSuperSeeder doesn't have letter-representation"),
        };
        write!(f, "{}", letter)
    }
}

pub struct SteamID3 {
    pub id_type: IDType,
    pub universe: Universe,
    pub account_number: u32,
}

impl FromStr for SteamID3 {
    type Err = SteamIDError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_type = s[1..2].parse::<IDType>()?;
        let universe = s[3..4].parse::<Universe>()?;
        let account_number = s[5..s.len() - 1].parse::<u32>()?;

        Ok(SteamID3 {
            id_type,
            universe,
            account_number,
        })
    }
}

impl fmt::Display for SteamID3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}:{}]",
            self.id_type, self.universe as u8, self.account_number
        )
    }
}

impl TryFrom<SteamID> for SteamID3 {
    type Error = SteamIDError;

    fn try_from(id: SteamID) -> Result<Self, Self::Error> {
        let universe = ((id.0 >> 56) as u8).try_into()?;
        let id_type = (((id.0 << 8) >> 52) as u8).try_into()?;
        let account_number = (id.0 >> 32) as u32;
        Ok(SteamID3 {
            id_type,
            universe,
            account_number,
        })
    }
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
