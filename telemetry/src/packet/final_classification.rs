use super::{Attributes, FromBytes};

/// Final Classification Packet
///
/// This packet details the final classification at the end of the race,
/// matching the post-race results screen. This is especially useful for multiplayer gmaes,
/// where it might not be possible to send lap times on the final frame due to network delay.
///
/// Frequency: Once at the end of a race  
/// Size: 1020 bytes  
/// Version: 1
#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct PacketFinalClassificationData {
    /// Header
    pub header: super::PacketHeader,
    /// Number of cars in the final classification
    pub num_cars: u8,
    /// Final classification data for all cars
    pub classification_data: [FinalClassificationData; 22],
}

impl FromBytes for PacketFinalClassificationData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketFinalClassificationData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct FinalClassificationData {
    /// Finishing position
    pub position: u8,
    /// Number of laps completed
    pub num_laps: u8,
    /// Grid position of the car
    pub grid_position: u8,
    /// Number of points scored
    pub points: u8,
    /// Number of pit stops made
    pub num_pit_stops: u8,
    /// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = did not finish,
    /// 5 = disqualified, 6 = not classified, 7 = retired
    pub result_status: u8,
    /// Best lap time of the session in milliseconds
    pub best_lap_time_in_ms: u32,
    /// Total race time in seconds without penalties
    pub total_race_time: f64,
    /// Total penalties accumulated in seconds
    pub penalties_time: u8,
    /// Number of penalties applied to this driver
    pub num_penalties: u8,
    /// Number of tyre stints
    pub num_tyre_stints: u8,
    /// Actual tyres used by this driver
    pub tyre_stints_actual: [u8; 8],
    /// Visual tyres used by this driver
    pub tyre_stints_visual: [u8; 8],
    /// The lap number stints end on
    pub tyre_stints_end_laps: [u8; 8],
}
