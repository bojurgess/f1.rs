use serde::{Deserialize, Serialize};

use super::{FromBytes, PacketError};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(C, packed)]
pub struct CarMotionData {
    /// World space X position - metres
    pub world_position_x: f32,
    /// World space Y position
    pub world_position_y: f32,
    /// World space Z position
    pub world_position_z: f32,
    /// Velocity in world space X – metres/s
    pub world_velocity_x: f32,
    /// Velocity in world space Y
    pub world_velocity_y: f32,
    /// Velocity in world space Z
    pub world_velocity_z: f32,
    /// World space forward X direction (normalised)
    pub world_forward_dir_x: i16,
    /// World space forward Y direction (normalised)
    pub world_forward_dir_y: i16,
    /// World space forward Z direction (normalised)
    pub world_forward_dir_z: i16,
    /// World space right X direction (normalised)
    pub world_right_dir_x: i16,
    /// World space right Y direction (normalised)
    pub world_right_dir_y: i16,
    /// World space right Z direction (normalised)
    pub world_right_dir_z: i16,
    /// Lateral G-Force component
    pub g_force_lateral: f32,
    /// Longitudinal G-Force component
    pub g_force_longitudinal: f32,
    /// Vertical G-Force component
    pub g_force_vertical: f32,
    /// Yaw angle in radians
    pub m_yaw: f32,
    /// Pitch angle in radians
    pub m_pitch: f32,
    /// Roll angle in radians
    pub m_roll: f32,
}

/// # Motion Packet
///
/// The motion packet gives physics data for all the cars being driven.  
/// *N.B. For the normalised vectors below, to convert to float values divide by 32767.0f -*
/// *16-bit signed values are used to pack the data and on the assumption that direction values are always between -1.0f
/// and 1.0f*
///
/// Frequency: Rate as specified in menus  
/// Size: 1349 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[repr(C, packed)]
pub struct PacketMotionData {
    pub header: super::header::PacketHeader,
    pub car_motion_data: [CarMotionData; 22],
}

impl FromBytes for PacketMotionData {
    fn from_bytes(buf: &[u8]) -> Result<PacketMotionData, PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from::<_, PacketMotionData>(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}
