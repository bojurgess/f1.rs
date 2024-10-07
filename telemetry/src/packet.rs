mod header;

use std::fmt::Display;

pub use header::PacketHeader;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PacketID {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
    TyreSets,
    MotionEx,
}

impl From<u8> for PacketID {
    fn from(val: u8) -> PacketID {
        match val {
            0 => PacketID::Motion,
            1 => PacketID::Session,
            2 => PacketID::LapData,
            3 => PacketID::Event,
            4 => PacketID::Participants,
            5 => PacketID::CarSetups,
            6 => PacketID::CarTelemetry,
            7 => PacketID::CarStatus,
            8 => PacketID::FinalClassification,
            9 => PacketID::LobbyInfo,
            10 => PacketID::CarDamage,
            11 => PacketID::SessionHistory,
            12 => PacketID::TyreSets,
            13 => PacketID::MotionEx,
            _ => panic!("Invalid packet ID"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Packet {
    // testing purposes only
    Header(PacketHeader),
}

#[derive(Debug)]
pub enum PacketError {
    SerialisationError(Box<bincode::ErrorKind>),
    InvalidPacketID(u8),
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PacketError::SerialisationError(e) => write!(f, "Serialisation error: {:#?}", e),
            PacketError::InvalidPacketID(id) => write!(f, "Invalid packet ID: {}", id),
        }
    }
}

impl Packet {
    pub fn from_bytes(buf: &[u8]) -> Result<Packet, PacketError> {
        let header =
            PacketHeader::from_bytes(buf).map_err(|e| PacketError::SerialisationError(e))?;
        match PacketID::from(header.packet_id) {
            _ => Err(PacketError::InvalidPacketID(header.packet_id)),
        }
    }
}
