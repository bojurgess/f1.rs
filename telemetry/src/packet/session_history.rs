use serde_big_array::BigArray;

use super::{Attributes, FromBytes};

/// Session History Packet
///
/// This packet contains lap times and tyre usage for the session. It works slightly differently
/// from other packets. To reduce CPU and badnwidth, each packet relates to a specific vehicle
/// and is sent every 1/20s, cycling through cars. Therefore, in a 20-car race,
/// you should recieve an update for each vehicle at least once per second
///
/// Frequency: 20 per second but cycling through cars
/// Size: 1460 bytes
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct PacketSessionHistoryData {
    /// Header
    pub header: super::PacketHeader,
    /// Index of the car this lap data relates to
    pub car_idx: u8,
    /// Number of laps in the data
    pub num_laps: u8,
    /// Number of tyre stints in the data
    pub num_tyre_stints: u8,
    /// Lap with best lap time
    pub best_lap_time_lap_num: u8,
    /// Lap with best sector 1 time
    pub best_sector_1_lap_num: u8,
    /// Lap with best sector 2 time
    pub best_sector_2_lap_num: u8,
    /// Lap with best sector 3 time
    pub best_sector_3_lap_num: u8,
    /// Lap history data for 100 laps max
    #[serde(with = "BigArray")]
    pub lap_history_data: [LapHistoryData; 100],
    /// Tyre stint history data
    pub tyre_stints_history_data: [TyreStintHistoryData; 8],
}

impl FromBytes for PacketSessionHistoryData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketSessionHistoryData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct LapHistoryData {
    /// Lap time in milliseconds
    pub lap_time_in_ms: u32,
    /// Sector 1 time in milliseconds
    pub sector_1_time_in_ms: u16,
    /// Sector 1 time in minutes
    pub sector_1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    pub sector_2_time_in_ms: u16,
    /// Sector 2 time in minutes
    pub sector_2_time_minutes: u8,
    /// Sector 3 time in milliseconds
    pub sector_3_time_in_ms: u16,
    /// Sector 3 time in minutes
    pub sector_3_time_minutes: u8,
    // is this in the appendix?
    /// Lap validity flags (0x01 = lap valid, etc.)
    pub lap_valid_bit_flags: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct TyreStintHistoryData {
    /// Lap the tyre usage ends on (255 if current tyre)
    end_lap: u8,
    /// Actual tyres used
    tyre_actual_compound: u8,
    /// Visual tyres used
    tyre_visual_compound: u8,
}
