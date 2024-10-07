use super::{Attributes, FromBytes, PacketHeader, PacketID};
use serde_big_array::BigArray;

/// # Session Packet
///
/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second  
/// Size: 644 bytes  
/// Version: 1  
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketSessionData {
    /// Packet header information.
    pub header: PacketHeader,

    /// Weather:
    /// - 0 = clear
    /// - 1 = light cloud
    /// - 2 = overcast
    /// - 3 = light rain
    /// - 4 = heavy rain
    /// - 5 = storm
    pub weather: u8,

    /// Track temperature in degrees Celsius.
    pub track_temperature: i8,

    /// Air temperature in degrees Celsius.
    pub air_temperature: i8,

    /// Total number of laps in this race.
    pub total_laps: u8,

    /// Track length in meters.
    pub track_length: u16,

    /// Session type:
    /// - 0 = unknown
    /// - 1 = P1
    /// - 2 = P2
    /// - 3 = P3
    /// - 4 = Short P
    /// - 5 = Q1
    /// - 6 = Q2
    /// - 7 = Q3
    /// - 8 = Short Q
    /// - 9 = OSQ
    /// - 10 = R
    /// - 11 = R2
    /// - 12 = R3
    /// - 13 = Time Trial
    pub session_type: u8,

    /// Track ID, `-1` for unknown (see appendix).
    pub track_id: i8,

    /// Formula type:
    /// - 0 = F1 Modern
    /// - 1 = F1 Classic
    /// - 2 = F2
    /// - 3 = F1 Generic
    /// - 4 = Beta
    /// - 5 = Supercars
    /// - 6 = Esports
    /// - 7 = F2 2021
    pub formula: u8,

    /// Time left in session (seconds).
    pub session_time_left: u16,

    /// Total session duration (seconds).
    pub session_duration: u16,

    /// Pit speed limit in kilometers per hour.
    pub pit_speed_limit: u8,

    /// Whether the game is paused (network game only).
    pub game_paused: u8,

    /// Whether the player is spectating.
    pub is_spectating: u8,

    /// Index of the car being spectated.
    pub spectator_car_index: u8,

    /// SLI Pro support, `0` = inactive, `1` = active.
    pub sli_pro_native_support: u8,

    /// Number of marshal zones.
    pub num_marshal_zones: u8,

    /// List of marshal zones (max 21).
    pub marshal_zones: [MarshalZone; 21],

    /// Safety car status:
    /// - 0 = no safety car
    /// - 1 = full
    /// - 2 = virtual
    /// - 3 = formation lap
    pub safety_car_status: u8,

    /// Whether the game is online (0 = offline, 1 = online).
    pub network_game: u8,

    /// Number of weather forecast samples to follow.
    pub num_weather_forecast_samples: u8,

    /// Array of weather forecast samples (max 56).
    #[serde(with = "BigArray")]
    pub weather_forecast_samples: [WeatherForecastSample; 56],

    /// Forecast accuracy:
    /// - 0 = Perfect
    /// - 1 = Approximate
    pub forecast_accuracy: u8,

    /// AI Difficulty rating (0-110).
    pub ai_difficulty: u8,

    /// Identifier for season (persists across saves).
    pub season_link_identifier: u32,

    /// Identifier for weekend (persists across saves).
    pub weekend_link_identifier: u32,

    /// Identifier for session (persists across saves).
    pub session_link_identifier: u32,

    /// Ideal lap to pit for current strategy (player).
    pub pit_stop_window_ideal_lap: u8,

    /// Latest lap to pit for current strategy (player).
    pub pit_stop_window_latest_lap: u8,

    /// Predicted position to rejoin at (player).
    pub pit_stop_rejoin_position: u8,

    /// Steering assist (0 = off, 1 = on).
    pub steering_assist: u8,

    /// Braking assist:
    /// - 0 = off
    /// - 1 = low
    /// - 2 = medium
    /// - 3 = high
    pub braking_assist: u8,

    /// Gearbox assist:
    /// - 1 = manual
    /// - 2 = manual & suggested gear
    /// - 3 = auto
    pub gearbox_assist: u8,

    /// Pit assist (0 = off, 1 = on).
    pub pit_assist: u8,

    /// Pit release assist (0 = off, 1 = on).
    pub pit_release_assist: u8,

    /// ERS assist (0 = off, 1 = on).
    pub ers_assist: u8,

    /// DRS assist (0 = off, 1 = on).
    pub drs_assist: u8,

    /// Dynamic racing line:
    /// - 0 = off
    /// - 1 = corners only
    /// - 2 = full
    pub dynamic_racing_line: u8,

    /// Dynamic racing line type:
    /// - 0 = 2D
    /// - 1 = 3D
    pub dynamic_racing_line_type: u8,

    /// Game mode ID (see appendix).
    pub game_mode: u8,

    /// Ruleset (see appendix).
    pub rule_set: u8,

    /// Local time of day (minutes since midnight).
    pub time_of_day: u32,

    /// Session length:
    /// - 0 = None
    /// - 2 = Very Short
    /// - 3 = Short
    /// - 4 = Medium
    /// - 5 = Medium Long
    /// - 6 = Long
    /// - 7 = Full
    pub session_length: u8,

    /// Speed units for the lead player:
    /// - 0 = MPH
    /// - 1 = KPH
    pub speed_units_lead_player: u8,

    /// Temperature units for the lead player:
    /// - 0 = Celsius
    /// - 1 = Fahrenheit
    pub temperature_units_lead_player: u8,

    /// Speed units for the secondary player:
    /// - 0 = MPH
    /// - 1 = KPH
    pub speed_units_secondary_player: u8,

    /// Temperature units for the secondary player:
    /// - 0 = Celsius
    /// - 1 = Fahrenheit
    pub temperature_units_secondary_player: u8,

    /// Number of safety car periods during the session.
    pub num_safety_car_periods: u8,

    /// Number of virtual safety car periods during the session.
    pub num_virtual_safety_car_periods: u8,

    /// Number of red flags called during the session.
    pub num_red_flag_periods: u8,
}

impl FromBytes for PacketSessionData {
    fn from_bytes(bytes: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(bytes);
        match bincode::deserialize_from::<_, PacketSessionData>(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketSessionData {
    fn header(&self) -> PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> PacketID {
        self.header.packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct MarshalZone {
    /// Fraction (0..1) of way through the lap the marshal zone starts
    pub zone_start: f32,
    /// Flag types for the marshal zone:
    /// - -1 = invalid/unknown,
    /// - 0 = none,
    /// - 1 = green,
    /// - 2 = blue,
    /// - 3 = yellow
    pub zone_flag: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct WeatherForecastSample {
    /// The session type:
    /// - 0 = unknown
    /// - 1 = P1
    /// - 2 = P2
    /// - 3 = P3
    /// - 4 = Short P
    /// - 5 = Q1
    /// - 6 = Q2
    /// - 7 = Q3
    /// - 8 = Short Q
    /// - 9 = OSQ
    /// - 10 = R
    /// - 11 = R2
    /// - 12 = R3
    /// - 13 = Time Trial
    pub session_type: u8,

    /// Time in minutes the forecast is for.
    pub time_offset: u8,

    /// The weather:
    /// - 0 = clear
    /// - 1 = light cloud
    /// - 2 = overcast
    /// - 3 = light rain
    /// - 4 = heavy rain
    /// - 5 = storm
    pub weather: u8,

    /// Track temperature in degrees Celsius.
    pub track_temperature: i8,

    /// Track temperature change:
    /// - 0 = up
    /// - 1 = down
    /// - 2 = no change
    pub track_temperature_change: i8,

    /// Air temperature in degrees Celsius.
    pub air_temperature: i8,

    /// Air temperature change:
    /// - 0 = up
    /// - 1 = down
    /// - 2 = no change
    pub air_temperature_change: i8,

    /// Rain percentage (0-100).
    pub rain_percentage: u8,
}
