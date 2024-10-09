use super::{Attributes, FromBytes};

/// # Motion Ex Packet
///
/// The motion packet gives extended data for the car being driven
/// with the goal of driving a motion platform setup.
///
/// Frequency: Rate as specified in menus  
/// Size: 217 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketMotionExData {
    /// Header
    pub header: super::PacketHeader,
    /// Suspension position (RL, RR, FL, FR)
    pub suspension_position: [f32; 4],
    /// Suspension velocity (RL, RR, FL, FR)
    pub suspension_velocity: [f32; 4],
    /// Suspension acceleration (RL, RR, FL, FR)
    pub suspension_acceleration: [f32; 4],
    /// Wheel speed (RL, RR, FL, FR)
    pub wheel_speed: [f32; 4],
    /// Slip ratio for each wheel
    pub wheel_slip_ratio: [f32; 4],
    /// Slip angle for each wheel
    pub wheel_slip_angle: [f32; 4],
    /// Lateral force for each wheel
    pub wheel_lat_force: [f32; 4],
    /// Longitudinal force for each wheel
    pub wheel_long_force: [f32; 4],
    /// Height of center of gravity above ground
    pub height_of_cog_above_ground: f32,
    /// Local velocity in X axis (m/s)
    pub local_velocity_x: f32,
    /// Local velocity in Y axis (m/s)
    pub local_velocity_y: f32,
    /// Local velocity in Z axis (m/s)
    pub local_velocity_z: f32,
    /// Angular velocity X component (radians/s)
    pub angular_velocity_x: f32,
    /// Angular velocity Y component
    pub angular_velocity_y: f32,
    /// Angular velocity Z
    pub angular_velocity_z: f32,
    /// Angular acceleration X component (radians/s²)
    pub angular_acceleration_x: f32,
    /// Angular acceleration Y component
    pub angular_acceleration_y: f32,
    /// Angular acceleration Z component
    pub angular_acceleration_z: f32,
    /// Current front wheels angle (radians)
    pub front_wheels_angle: f32,
    /// Vertical force for each wheel
    pub wheel_vert_force: [f32; 4],
}

impl FromBytes for PacketMotionExData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl Attributes for PacketMotionExData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header().packet_id.into()
    }
}
