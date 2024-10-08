use serde_big_array::BigArray;

/// # Participants Packet
///
/// This is a list of participants in the race. If the vehicle is controlled by AI,
/// then the name will be the driver name. In multiplayer, the names will be the
/// Steam ID on PC or the LAN name if appropriate. The array should be indexed by
/// the vehicle index.
///
/// Frequency: Every 5 seconds  
/// Size: 1306 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketParticipantsData {
    /// Header
    pub header: super::PacketHeader,
    /// Number of active cars in the data – should match number of cars on HUD
    pub num_active_cars_u8: u8,
    pub participants: [ParticipantData; 22],
}

impl super::FromBytes for PacketParticipantsData {
    fn from_bytes(buf: &[u8]) -> Result<Self, super::PacketError> {
        let cursor = std::io::Cursor::new(buf);
        match bincode::deserialize_from::<_, PacketParticipantsData>(cursor) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(e.into()),
        }
    }
}

impl super::Attributes for PacketParticipantsData {
    fn header(&self) -> super::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> super::PacketID {
        self.header.packet_id.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct ParticipantData {
    /// Whether the vehicle is AI (1) or Human (0) controlled
    pub ai_controlled: u8,
    /// Driver ID - see appendix, 255 if network human
    pub driver_id: u8,
    /// Network ID – unique identifier for network players
    pub network_id: u8,
    /// Team ID - see appendix
    pub team_id: u8,
    /// My team flag – 1 = My Team, 0 = otherwise
    pub my_team: u8,
    /// Race number of the car
    pub race_number: u8,
    /// Nationality of the driver
    pub nationality: u8,
    /// Name of participant in UTF-8 format – null terminated
    /// Will be truncated with … (U+2026) if too long
    #[serde(with = "BigArray")]
    pub name: [u8; 48],
    /// The player's UDP setting 0 = restricted, 1 = public
    pub your_telemetry: u8,
    /// The player's show online names setting, 0 = off, 1 = on
    pub show_online_names: u8,
    /// 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
    pub platform: u8,
}
