use serde_big_array::BigArray;

use super::{Attributes, FromBytes};

/// # Lobby Info Packet
///
/// This packet details the players currently in a multiplayer lobby, including each player's selected car,
/// any AI involved in the game, and the ready status of each participant.
///
/// Frequency: Two every second when in the lobby  
/// Size: 1218 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct PacketLobbyInfoData {
    /// Header
    pub header: super::PacketHeader,
    /// Number of players in the lobby data
    pub num_players: u8,
    /// Lobby info for all players
    pub lobby_players: [LobbyInfoData; 22],
}

impl FromBytes for PacketLobbyInfoData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketLobbyInfoData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct LobbyInfoData {
    /// whether the vehicle is AI (1) or Human (0) controlled
    ai_controlled: u8,
    /// Team ID - see appendix (255 if no team selected)
    team_id: u8,
    /// Nationality of the driver
    nationality: u8,
    /// Platform (1 = Steam, 3 = Playstation, 4 = Xbox, 6 = Origin, 255 = unknown)
    platform: u8,
    /// Name of participant in UTF-8 format - null terminated;
    /// will be truncated with ... (U+2026) if too long
    #[serde(with = "BigArray")]
    name: [u8; 48],
    /// Car number of the player
    car_number: u8,
    /// 0 = not ready, 1 = ready, 2 = spectating
    ready_status: u8,
}
