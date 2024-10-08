use bincode::deserialize_from;

use super::{Attributes, FromBytes, PacketError, PacketHeader};

/// Event Packet
///
/// This packet gives details of events that happen during the course of a session.
///
/// Frequency: When the event occurs  
/// Size: 45 bytes  
/// Version: 1
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(C, packed)]
pub struct PacketEventData {
    pub header: PacketHeader,
    pub event_string_code: [u8; 4],
    pub event_details: EventDataDetails,
}

impl Attributes for PacketEventData {
    fn header(&self) -> crate::packet::PacketHeader {
        self.header.clone()
    }

    fn packet_id(&self) -> crate::packet::PacketID {
        self.header().packet_id.into()
    }
}

impl FromBytes for PacketEventData {
    fn from_bytes(buf: &[u8]) -> Result<Self, crate::packet::PacketError> {
        let mut cursor = std::io::Cursor::new(buf);

        let header = bincode::deserialize_from::<_, PacketHeader>(&mut cursor)?;
        let header_size = std::mem::size_of::<PacketHeader>();

        let string_code_buf = &buf[header_size..header_size + 4];
        let event_string_code: [u8; 4] = string_code_buf
            .try_into()
            .map_err(|_| PacketError::EventDecodeError())?;

        let string_code =
            std::str::from_utf8(&event_string_code).map_err(|_| PacketError::EventDecodeError())?;

        let event_details = match string_code {
            "FTLP" => EventDataDetails::FastestLap(deserialize_from::<_, FastestLap>(&mut cursor)?),
            "RTMT" => EventDataDetails::Retirement(deserialize_from::<_, Retirement>(&mut cursor)?),
            "TMPT" => EventDataDetails::TeamMateInPits(deserialize_from::<_, TeamMateInPits>(
                &mut cursor,
            )?),
            "RCWN" => EventDataDetails::RaceWinner(deserialize_from::<_, RaceWinner>(&mut cursor)?),
            "PENL" => EventDataDetails::Penalty(deserialize_from::<_, Penalty>(&mut cursor)?),
            "SPTP" => EventDataDetails::SpeedTrap(deserialize_from::<_, SpeedTrap>(&mut cursor)?),
            "STLG" => {
                EventDataDetails::StartLights(deserialize_from::<_, StartLights>(&mut cursor)?)
            }
            "DTPN" => EventDataDetails::DriveThroughPenaltyServed(deserialize_from::<
                _,
                DriveThroughPenaltyServed,
            >(&mut cursor)?),
            "SGPN" => EventDataDetails::StopGoPenaltyServed(deserialize_from::<
                _,
                StopGoPenaltyServed,
            >(&mut cursor)?),
            "FLBK" => EventDataDetails::Flashback(deserialize_from::<_, Flashback>(&mut cursor)?),
            "BUTN" => EventDataDetails::Buttons(deserialize_from::<_, Buttons>(&mut cursor)?),
            "OVTK" => EventDataDetails::Overtake(deserialize_from::<_, Overtake>(&mut cursor)?),
            "SSTA" => EventDataDetails::SessionStarted,
            "SEND" => EventDataDetails::SessionEnded,
            "DRSE" => EventDataDetails::DRSEnabled,
            "DRSD" => EventDataDetails::DRSDisabled,
            "CHQF" => EventDataDetails::ChequeredFlag,
            "LGOT" => EventDataDetails::LightsOut,
            "REDL" => EventDataDetails::RedFlag,
            _ => return Err(PacketError::EventCodeOutOfBounds(string_code.len())),
        };

        Ok(PacketEventData {
            header,
            event_string_code,
            event_details,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(u8)]
pub enum PenaltyType {
    DriveThrough,
    StopGo,
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulation,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(u8)]
pub enum InfringementType {
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGaintedTimeMinor,
    CornerCuttingRanWideGaintedTimeSignificant,
    CornerCuttingRanWideGaintedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    FormationLapParking,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    ParcFermeChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPistop,
    AttributeAssigned,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(u8)]
pub enum EventDataDetails {
    FastestLap(FastestLap),
    Retirement(Retirement),
    TeamMateInPits(TeamMateInPits),
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    Flashback(Flashback),
    Buttons(Buttons),
    Overtake(Overtake),
    SessionStarted,
    SessionEnded,
    DRSEnabled,
    DRSDisabled,
    ChequeredFlag,
    LightsOut,
    RedFlag,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct FastestLap {
    /// Vehicle index of car achieving fastest lap
    pub vehicle_idx: u8,
    /// Lap time is in seconds
    pub lap_time: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct Retirement {
    /// Vehicle index of car retiring
    pub vehicle_idx: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct TeamMateInPits {
    /// Vehicle index of team mate
    pub vehicle_idx: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct RaceWinner {
    /// Vehicle index of the race winner
    pub vehicle_idx: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct Penalty {
    /// Penalty type – see Appendices
    pub penalty_type: PenaltyType,
    /// Infringement type – see Appendices
    pub infringement_type: InfringementType,
    /// Vehicle index of the car the penalty is applied to
    pub vehicle_idx: u8,
    /// Vehicle index of the other car involved
    pub other_vehicle_idx: u8,
    /// Time gained or time spent doing action in seconds
    pub time: u8,
    /// Lap the penalty occurred on
    pub lap_num: u8,
    /// Number of places gained by this
    pub places_gained: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct SpeedTrap {
    /// Vehicle index of the vehicle triggering speed trap
    pub vehicle_idx: u8,
    /// Top speed achieved in kilometres per hour
    pub speed: f32,
    /// Overall fastest speed in session = 1 otherwise 0
    pub is_overall_fastest_in_session: u8,
    /// Fastest speed for driver in session = 1 otherwise 0
    pub is_driver_fastest_in_session: u8,
    /// Vehicle index of the vehicle that is the fastest
    pub fastest_vehicle_idx_in_session: u8,
    /// Speed of the vehicle that is the fastest
    pub fastest_speed_in_session: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct StartLights {
    /// Number of lights showing
    pub num_lights: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct DriveThroughPenaltyServed {
    /// Vehicle index of the vehicle serving the drive-through penalty
    pub vehicle_idx: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct StopGoPenaltyServed {
    /// Vehicle index of the vehicle serving stop-go penalty
    pub vehicle_idx: u8,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct Flashback {
    /// Frame identifier flashed back to
    pub flashback_frame_identifier: u32,
    /// Session time flashed back to
    pub flashback_session_time: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct Buttons {
    /// Bit flags specifying which buttons are being pressed
    pub button_status: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(C, packed)]
pub struct Overtake {
    /// Vehicle index of the vehicle overtaking
    pub overtaking_vehicle_idx: u8,
    /// Vehicle index of the vehicle being overtaken
    pub being_overtaken_vehicle_idx: u8,
}
