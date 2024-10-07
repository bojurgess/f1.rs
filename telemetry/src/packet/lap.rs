use super::{Attributes, FromBytes, PacketError, PacketHeader, PacketID};

/// # Lap Data Packet
/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus  
/// Size: 1131 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketLapData {
    pub header: PacketHeader,
    pub lap_data: [LapData; 22],
    pub time_trial_personal_best_car_idx: u8,
    pub time_trial_rival_car_idx: u8,
}

impl FromBytes for PacketLapData {
    fn from_bytes(buf: &[u8]) -> Result<PacketLapData, PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from::<_, PacketLapData>(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketLapData {
    fn header(&self) -> PacketHeader {
        self.header
    }

    fn packet_id(&self) -> PacketID {
        self.header.packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct LapData {
    /// Last lap time in milliseconds
    last_lap_time_in_ms: u32,
    /// Current time around the lap in milliseconds
    current_lap_time_in_ms: u32,
    /// Sector 1 time in milliseconds
    sector1_time_in_ms: u16,
    /// Sector 1 whole minute part
    sector1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    sector2_time_in_ms: u16,
    /// Sector 2 whole minute part
    sector2_time_minutes: u8,
    /// Time delta to car in front in milliseconds
    delta_to_car_in_front_in_ms: u16,
    /// Time delta to race leader in milliseconds
    delta_to_race_leader_in_ms: u16,
    /// Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    lap_distance: f32,
    /// Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    total_distance: f32,
    /// Delta in seconds for safety car
    safety_car_delta: f32,
    /// Car race position
    car_position: u8,
    /// Current lap number
    current_lap_num: u8,
    /// 0 = none, 1 = pitting, 2 = in pit area
    pit_status: u8,
    /// Number of pit stops taken in this race
    num_pit_stops: u8,
    /// 0 = sector1, 1 = sector2, 2 = sector3
    sector: u8,
    /// Current lap invalid - 0 = valid, 1 = invalid
    current_lap_invalid: bool,
    /// Accumulated time penalties in seconds to be added
    penalties: u8,
    /// Accumulated number of warnings issued
    total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued
    corner_cutting_warnings: u8,
    /// Num drive through pens left to serve
    num_unserved_drive_through_pens: u8,
    /// Num stop go pens left to serve
    num_unserved_stop_go_pens: u8,
    /// Grid position the vehicle started the race in
    grid_position: u8,
    /// Status of driver - 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    driver_status: u8,
    /// Result status - 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = didnotfinish, 5 = disqualified, 6 = not classified, 7 = retired
    result_status: u8,
    /// Pit lane timing, 0 = inactive, 1 = active
    pit_lane_timer_active: bool,
    /// If active, the current time spent in the pit lane in ms
    pit_lane_time_in_lane_in_ms: u16,
    /// Time of the actual pit stop in ms
    pit_stop_timer_in_ms: u16,
    /// Whether the car should serve a penalty at this stop
    pit_stop_should_serve_pen: bool,
}
