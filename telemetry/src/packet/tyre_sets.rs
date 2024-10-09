use super::{Attributes, FromBytes};

/// # Tyre Sets Packet
///
/// This packet gives more in-depth details about tyre-sets assigned to a vehicle during the session.
///
/// Frequency: 20 per second but cycling through cars  
/// Size: 231 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct PacketTyreSetData {
    /// Header
    pub header: super::PacketHeader,
    /// Index of the car this data relates to
    pub car_idx: u8,
    /// Data for 13 (dry) + 7 (wet) tyres
    pub tyre_set_data: [TyreSetData; 20],
    /// Index into array of fitted tyre
    pub fitted_idx: u8,
}

impl FromBytes for PacketTyreSetData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketTyreSetData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct TyreSetData {
    /// Actual tyre compound used
    pub actual_tyre_compound: u8,
    /// Visual tyre compound used
    pub visual_tyre_compound: u8,
    /// Tyre wear (percentage)
    pub wear: u8,
    /// Whether this set is currently available
    pub available: u8,
    /// Recommended session for tyre set
    pub recommended_session: u8,
    /// Laps left in this tyre set
    pub life_span: u8,
    /// Max laps recommended for this compound
    pub usable_life: u8,
    /// Lap delta time compared to fitted set (milliseconds)
    pub lap_delta_time: i16,
    /// Whether the set is fitted (0 = not fitted, 1 = fitted)
    pub fitted: u8,
}
