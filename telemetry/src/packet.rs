mod car_damage;
mod car_setups;
mod car_status;
mod car_telemetry;
mod event;
mod final_classification;
mod header;
mod lap;
mod lobby_info;
mod motion;
mod motion_ex;
mod participants;
mod session;
mod session_history;
mod tyre_sets;

use std::fmt::Display;

pub use car_damage::PacketCarDamageData;
pub use car_setups::PacketCarSetupData;
pub use car_status::PacketCarStatusData;
pub use car_telemetry::PacketCarTelemetryData;
pub use event::PacketEventData;
pub use final_classification::PacketFinalClassificationData;
pub use header::PacketHeader;
pub use lap::PacketLapData;
pub use lobby_info::PacketLobbyInfoData;
pub use motion::PacketMotionData;
use motion_ex::PacketMotionExData;
pub use participants::PacketParticipantsData;
pub use session::PacketSessionData;
pub use session_history::PacketSessionHistoryData;
use tyre_sets::PacketTyreSetData;

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
            // Something catastrophically wrong has happened if this code path executes
            // (just saying)
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
    CarTelemetry(PacketCarTelemetryData),
    CarStatus(PacketCarStatusData),
    FinalClassification(PacketFinalClassificationData),
    LobbyInfo(PacketLobbyInfoData),
    CarDamage(PacketCarDamageData),
    SessionHistory(PacketSessionHistoryData),
    TyreSets(PacketTyreSetData),
    MotionEx(PacketMotionExData),
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
            PacketID::CarTelemetry => Ok(Packet::CarTelemetry(PacketCarTelemetryData::from_bytes(
                buf,
            )?)),
            PacketID::CarStatus => Ok(Packet::CarStatus(PacketCarStatusData::from_bytes(buf)?)),
            PacketID::FinalClassification => Ok(Packet::FinalClassification(
                PacketFinalClassificationData::from_bytes(buf)?,
            )),
            PacketID::LobbyInfo => Ok(Packet::LobbyInfo(PacketLobbyInfoData::from_bytes(buf)?)),
            PacketID::CarDamage => Ok(Packet::CarDamage(PacketCarDamageData::from_bytes(buf)?)),
            PacketID::SessionHistory => Ok(Packet::SessionHistory(
                PacketSessionHistoryData::from_bytes(buf)?,
            )),
            PacketID::TyreSets => Ok(Packet::TyreSets(PacketTyreSetData::from_bytes(buf)?)),
            PacketID::MotionEx => Ok(Packet::MotionEx(PacketMotionExData::from_bytes(buf)?)),
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
            Packet::CarTelemetry(data) => data.header(),
            Packet::CarStatus(data) => data.header(),
            Packet::FinalClassification(data) => data.header(),
            Packet::LobbyInfo(data) => data.header(),
            Packet::CarDamage(data) => data.header(),
            Packet::SessionHistory(data) => data.header(),
            Packet::TyreSets(data) => data.header(),
            Packet::MotionEx(data) => data.header(),
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
            Packet::CarTelemetry(data) => data.packet_id(),
            Packet::CarStatus(data) => data.packet_id(),
            Packet::FinalClassification(data) => data.packet_id(),
            Packet::LobbyInfo(data) => data.packet_id(),
            Packet::CarDamage(data) => data.packet_id(),
            Packet::SessionHistory(data) => data.packet_id(),
            Packet::TyreSets(data) => data.packet_id(),
            Packet::MotionEx(data) => data.packet_id(),
        }
    }
}
