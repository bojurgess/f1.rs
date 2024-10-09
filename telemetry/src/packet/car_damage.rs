use super::{Attributes, FromBytes};

/// # Car Damage Packet
///
/// This packet details car damage parameters for all the cars in the race.
///
/// Frequency: 10 per second  
/// Size: 953 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct PacketCarDamageData {
    /// Header
    pub header: super::PacketHeader,
    /// Damage data for all cars
    pub car_damage_data: [CarDamageData; 22],
}

impl FromBytes for PacketCarDamageData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketCarDamageData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct CarDamageData {
    /// Tyre wear (percentage)
    pub tyres_wear: [f32; 4],
    /// Tyre damage (percentage)
    pub tyres_damage: [u8; 4],
    /// Brakes damage (percentage)
    pub brakes_damage: [u8; 4],
    /// Front left wing damage (percentage)
    pub front_left_wing_damage: u8,
    /// Front right wing damage (percentage)
    pub front_right_wing_damage: u8,
    /// Rear wing damage (percentage)
    pub rear_wing_damage: u8,
    /// Floor damage (percentage)
    pub floor_damage: u8,
    /// Diffuser damage (percentage)
    pub diffuser_damage: u8,
    /// Sidepod damage (percentage)
    pub sidepod_damage: u8,
    /// DRS fault indicator (0 = OK, 1 = fault)
    pub drs_fault: u8,
    /// ERS fault indicator (0 = OK, 1 = fault)
    pub ers_fault: u8,
    /// Gearbox damage (fault)
    pub gear_box_damage: u8,
    /// Engine damage (percentage)
    pub engine_damage: u8,
    /// MGU-H wear (percentage)
    pub engine_mgu_h_wear: u8,
    /// Energy store wear (percentage)
    pub engine_es_wear: u8,
    /// CE wear (percentage)
    pub engine_ce_wear: u8,
    /// ICE wear (percentage)
    pub engine_ice_wear: u8,
    /// MGU-K wear (percentage)
    pub engine_mgu_k_wear: u8,
    /// Turbocharger wear (percentage)
    pub engine_tc_wear: u8,
    /// Engine blown (0 = OK, 1 = blown)
    pub engine_blown: u8,
    /// Engine seized (0 = OK, 1 = seized)
    pub engine_seized: u8,
}
