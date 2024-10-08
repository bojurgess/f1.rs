mod car_setups;
mod event;
mod header;
mod lap;
mod motion;
mod participants;
mod session;

use std::fmt::Display;

pub use car_setups::PacketCarSetupData;
pub use event::PacketEventData;
pub use header::PacketHeader;
pub use lap::PacketLapData;
pub use motion::PacketMotionData;
pub use participants::PacketParticipantsData;
pub use session::PacketSessionData;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PacketID {
    Motion,
    Session,
    Lap,
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
            2 => PacketID::Lap,
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

    Motion(PacketMotionData),
    Session(PacketSessionData),
    Lap(PacketLapData),
    Event(PacketEventData),
    Participants(PacketParticipantsData),
    CarSetups(PacketCarSetupData),
}

#[derive(Debug)]
pub enum PacketError {
    SerialisationError(Box<bincode::ErrorKind>),
    InvalidPacketID(u8),
    EventCodeOutOfBounds(usize),
    EventDecodeError(),
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PacketError::SerialisationError(e) => write!(f, "Serialisation error: {:#?}", e),
            PacketError::InvalidPacketID(id) => write!(f, "Invalid packet ID: {}", id),
            PacketError::EventCodeOutOfBounds(id) => {
                write!(f, "Event code of length {} is out of bounds", id)
            }
            PacketError::EventDecodeError() => write!(f, "Failed to decode event data"),
        }
    }
}

pub trait FromBytes {
    fn from_bytes(buf: &[u8]) -> Result<Self, PacketError>
    where
        Self: Sized;
}

pub trait Attributes {
    fn header(&self) -> PacketHeader;
    fn packet_id(&self) -> PacketID;
}

// allows usage of `?` operator with `PacketError`
impl From<Box<bincode::ErrorKind>> for PacketError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        PacketError::SerialisationError(e)
    }
}

impl FromBytes for Packet {
    fn from_bytes(buf: &[u8]) -> Result<Packet, PacketError> {
        let header = PacketHeader::from_bytes(buf)?;

        match PacketID::from(header.packet_id) {
            PacketID::Motion => Ok(Packet::Motion(PacketMotionData::from_bytes(buf)?)),
            PacketID::Session => Ok(Packet::Session(PacketSessionData::from_bytes(buf)?)),
            PacketID::Lap => Ok(Packet::Lap(PacketLapData::from_bytes(buf)?)),
            PacketID::Event => Ok(Packet::Event(PacketEventData::from_bytes(buf)?)),
            PacketID::Participants => Ok(Packet::Participants(PacketParticipantsData::from_bytes(
                buf,
            )?)),
            PacketID::CarSetups => Ok(Packet::CarSetups(PacketCarSetupData::from_bytes(buf)?)),
            _ => Err(PacketError::InvalidPacketID(header.packet_id)),
        }
    }
}

impl Attributes for Packet {
    fn header(&self) -> PacketHeader {
        match self {
            Packet::Header(header) => header.clone(),
            Packet::Motion(data) => data.header(),
            Packet::Session(data) => data.header(),
            Packet::Lap(data) => data.header(),
            Packet::Event(data) => data.header(),
            Packet::Participants(data) => data.header(),
            Packet::CarSetups(data) => data.header(),
        }
    }

    fn packet_id(&self) -> PacketID {
        match self {
            Packet::Header(header) => header.packet_id(),
            Packet::Motion(data) => data.packet_id(),
            Packet::Session(data) => data.packet_id(),
            Packet::Lap(data) => data.packet_id(),
            Packet::Event(data) => data.packet_id(),
            Packet::Participants(data) => data.packet_id(),
            Packet::CarSetups(data) => data.packet_id(),
        }
    }
}
