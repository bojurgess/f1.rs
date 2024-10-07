mod header;

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

impl Packet {
    pub fn from_bytes(buf: &[u8]) -> Result<Packet, Box<bincode::ErrorKind>> {
        let header = PacketHeader::from_bytes(buf)?;
        match PacketID::from(header.packet_id) {
            _ => Ok(Packet::Header(header)),
        }
    }
}
