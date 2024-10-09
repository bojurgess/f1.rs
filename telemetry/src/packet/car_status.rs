use super::{Attributes, FromBytes, PacketID};

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketCarStatusData {
    /// Header
    header: super::PacketHeader,
    /// Status data for all cars
    car_status_data: [CarStatusData; 22],
}

impl FromBytes for PacketCarStatusData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketCarStatusData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        PacketID::from(self.header().packet_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct CarStatusData {
    /// Traction control level (0 = off, 1 = medium, 2 = full)
    pub traction_control: u8,
    /// ABS (0 = off, 1 = on)
    pub anti_lock_brakes: bool,
    /// Fuel mix (0 = lean, 1 = standard, 2 = rich, 3 = max)
    pub fuel_mix: u8,
    /// Front brake bias (percentage)
    pub front_brake_bias: bool,
    /// Pit limiter status (0 = off, 1 = on)
    pub pit_limiter_status: u8,
    /// Current fuel mass
    pub fuel_in_tank: f32,
    /// Fuel capacity,
    pub fuel_capacity: f32,
    /// Fuel remaining in laps (value on MFD)
    pub fuel_remaining_laps: f32,
    /// Car's max RPM point (rev limiter)
    pub max_rpm: u16,
    /// Car's idle RPM
    pub idle_rpm: u16,
    /// Maximum number of gears
    pub max_gears: u8,
    /// DRS allowed (0 = not allowed, 1 = allowed)
    pub drs_allowed: bool,
    /// DRS activation distance in meters (0 = DRS not available)
    pub drs_activation_distance: u16,
    /// F1 Modern: actual tyre compound, see Appendices
    pub actual_tyre_compound: u8,
    /// Visual tyre compound (could differ from actual)
    pub visual_tyre_compound: u8,
    /// Age in laps of the current set of tyres
    pub tyres_age_laps: u8,
    /// -1 = invalid, 0 = none, 1 = green, 2 = blue, 3 = yellow
    pub vehicle_fia_flags: i8,
    /// Engine power output of ICE (W)
    pub engine_power_ice: f32,
    /// Engine power output of MGU-K (W)
    pub engine_power_mgu_k: f32,
    /// ERS energy store in Joules
    pub ers_store_energy: f32,
    /// ERS deployment mode (0 = none, 1 = medium, 2 = hotlap, 3 = overtake)
    pub ers_deploy_mode: u8,
    /// ERS energy harvested this lap by MGU-K
    pub ers_harvested_this_lap_mgu_k: f32,
    /// ERS energy harvested this lap by MGU-H
    pub ers_harvested_this_lap_mgu_h: f32,
    /// ERS energy deployed this lap
    pub ers_deployed_this_lap: f32,
    /// Whether the car is paused in a network game
    pub network_paused: u8,
}
