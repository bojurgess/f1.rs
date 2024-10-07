use serde::{Deserialize, Serialize};

use super::{FromBytes, Header};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(C, packed)]
pub struct PacketHeader {
    /// Major revision of packet e.g. 2023
    pub packet_format: u16,
    /// Game year - last two digits e.g. 23
    pub game_year: u8,
    /// Game major version - "X.00"
    pub game_major_version: u8,
    /// Game minor version - "1.XX"
    pub game_minor_version: u8,
    /// Version of this packet type, all start from 1
    pub packet_version: u8,
    /// Identifier for the packet type, see UDP spec
    pub packet_id: u8,
    /// Unique identifier for the session
    pub session_uid: u64,
    /// Session timestamp
    pub session_time: f32,
    /// Identifier for the frame the data was retrieved on
    pub frame_identifier: u32,
    /// Overall identifier for the frame the data was retrieved on, doesn't go back after flashback
    pub overall_frame_identifier: u32,
    /// Index of player's car in the array
    pub player_car_index: u8,
    /// Index of secondary player's car in the array (splitscreen); 255 if no second player
    pub secondary_player_car_index: u8,
}

impl FromBytes for PacketHeader {
    fn from_bytes(buf: &[u8]) -> Result<PacketHeader, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from::<_, PacketHeader>(cursor) {
            Ok(header) => Ok(header),
            Err(e) => Err(e.into()),
        }
    }
}

impl Header for PacketHeader {
    fn header(&self) -> PacketHeader {
        self.clone()
    }
}
