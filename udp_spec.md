# Data Output from F1® 23 Game

## Contents
- [Overview](#overview)
- [Packet Information](#packet-information)
- [FAQS](#faqs)
- [Appendices](#appendices)
- [Legal Notice](#legal-notice)

---

## Overview

The F1® 23 Game supports the output of certain game data across UDP connections. This data can be used to supply race information to external applications or to drive certain hardware (e.g. motion platforms, force feedback steering wheels, and LED devices).

The following information summarizes these data structures so that developers of supporting hardware or software can configure them to work correctly with the F1® 23 Game.

**Note**: To ensure that you are using the latest specification for this game, please check our official forum page [here](#).

If you cannot find the information that you require, please contact the team via the official forum thread listed above. For any bugs with the UDP system, please post a new bug report on the F1® 23 Game forum.

**DISCLAIMER**: “This information is being provided under license from EA for reference purposes only and we do not make any representations or warranties about the accuracy or reliability of the information for any specific purpose.”

---

## Packet Information

### Packet Types

Each packet carries different types of data rather than having one packet that contains everything. The header in each packet describes the packet type and versioning info, making it easier for applications to check they are interpreting the incoming data in the correct way. Please note that all values are encoded using Little Endian format. All data is packed.

The following data types are used in the structures:

| Type     | Description                              |
|----------|------------------------------------------|
| uint8    | Unsigned 8-bit integer                   |
| int8     | Signed 8-bit integer                     |
| uint16   | Unsigned 16-bit integer                  |
| int16    | Signed 16-bit integer                    |
| uint32   | Unsigned 32-bit integer                  |
| float    | Floating point (32-bit)                  |
| double   | Double-precision floating point (64-bit) |
| uint64   | Unsigned 64-bit integer                  |
| char     | Character                                |

### Packet Header

Each packet has the following header:

```c
struct PacketHeader {
    uint16    m_packetFormat;            // 2023
    uint8     m_gameYear;                // Game year - last two digits e.g. 23
    uint8     m_gameMajorVersion;        // Game major version - "X.00"
    uint8     m_gameMinorVersion;        // Game minor version - "1.XX"
    uint8     m_packetVersion;           // Version of this packet type, all start from 1
    uint8     m_packetId;                // Identifier for the packet type
    uint64    m_sessionUID;              // Unique identifier for the session
    float     m_sessionTime;             // Session timestamp
    uint32    m_frameIdentifier;         // Identifier for the frame the data was retrieved on
    uint32    m_overallFrameIdentifier;  // Overall identifier for the frame, doesn't reset after flashbacks
    uint8     m_playerCarIndex;          // Index of player's car in the array
    uint8     m_secondaryPlayerCarIndex; // Index of secondary player's car in the array (splitscreen), 255 if no second player
};
```

---

### Packet IDs

| Packet Name          | Value | Description                                                        |
|----------------------|-------|--------------------------------------------------------------------|
| Motion               | 0     | Contains all motion data for player’s car – only sent while player is in control |
| Session              | 1     | Data about the session – track time left                           |
| Lap Data             | 2     | Data about all the lap times of cars in the session                |
| Event                | 3     | Various notable events that happen during a session                |
| Participants         | 4     | List of participants in the session, mostly relevant for multiplayer |
| Car Setups           | 5     | Packet detailing car setups for cars in the race                   |
| Car Telemetry        | 6     | Telemetry data for all cars                                        |
| Car Status           | 7     | Status data for all cars                                           |
| Final Classification | 8     | Final classification confirmation at the end of a race             |
| Lobby Info           | 9     | Information about players in a multiplayer lobby                   |
| Car Damage           | 10    | Damage status for all cars                                         |
| Session History      | 11    | Lap and tyre data for the session                                  |
| Tyre Sets            | 12    | Extended tyre set data                                             |
| Motion Ex            | 13    | Extended motion data for player car                                |

---

### Motion Packet

The motion packet gives physics data for all the cars being driven.

N.B. For the normalized vectors below, to convert to float values divide by `32767.0f` – 16-bit signed values are used to pack the data on the assumption that direction values are always between `-1.0f` and `1.0f`.

- **Frequency**: Rate as specified in menus
- **Size**: 1349 bytes
- **Version**: 1

```c
struct CarMotionData {
    float         m_worldPositionX;           // World space X position - metres
    float         m_worldPositionY;           // World space Y position
    float         m_worldPositionZ;           // World space Z position
    float         m_worldVelocityX;           // Velocity in world space X – metres/s
    float         m_worldVelocityY;           // Velocity in world space Y
    float         m_worldVelocityZ;           // Velocity in world space Z
    int16         m_worldForwardDirX;         // World space forward X direction (normalized)
    int16         m_worldForwardDirY;         // World space forward Y direction (normalized)
    int16         m_worldForwardDirZ;         // World space forward Z direction (normalized)
    int16         m_worldRightDirX;           // World space right X direction (normalized)
    int16         m_worldRightDirY;           // World space right Y direction (normalized)
    int16         m_worldRightDirZ;           // World space right Z direction (normalized)
    float         m_gForceLateral;            // Lateral G-Force component
    float         m_gForceLongitudinal;       // Longitudinal G-Force component
    float         m_gForceVertical;           // Vertical G-Force component
    float         m_yaw;                      // Yaw angle in radians
    float         m_pitch;                    // Pitch angle in radians
    float         m_roll;                     // Roll angle in radians
};
```

```c
struct PacketMotionData {
    PacketHeader    m_header;                // Header
    CarMotionData   m_carMotionData[22];     // Data for all cars on track
};
```
---

### Session Packet

The session packet includes details about the current session in progress.

- **Frequency**: 2 per second
- **Size**: 644 bytes
- **Version**: 1

```c
struct MarshalZone {
    float  m_zoneStart;   // Fraction (0..1) of way through the lap the marshal zone starts
    int8   m_zoneFlag;    // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
};

struct WeatherForecastSample {
    uint8  m_sessionType;              // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
                                       // 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
                                       // 12 = R3, 13 = Time Trial
    uint8  m_timeOffset;               // Time in minutes the forecast is for
    uint8  m_weather;                  // Weather - 0 = clear, 1 = light cloud, 2 = overcast
                                       // 3 = light rain, 4 = heavy rain, 5 = storm
    int8   m_trackTemperature;         // Track temperature in degrees Celsius
    int8   m_trackTemperatureChange;   // Track temp. change – 0 = up, 1 = down, 2 = no change
    int8   m_airTemperature;           // Air temperature in degrees Celsius
    int8   m_airTemperatureChange;     // Air temp. change – 0 = up, 1 = down, 2 = no change
    uint8  m_rainPercentage;           // Rain percentage (0-100)
};

struct PacketSessionData {
    PacketHeader    m_header;               // Header

    uint8           m_weather;              // Weather - 0 = clear, 1 = light cloud, 2 = overcast
                                            // 3 = light rain, 4 = heavy rain, 5 = storm
    int8            m_trackTemperature;     // Track temp. in degrees Celsius
    int8            m_airTemperature;       // Air temp. in degrees Celsius
    uint8           m_totalLaps;            // Total number of laps in this race
    uint16          m_trackLength;          // Track length in meters
    uint8           m_sessionType;          // 0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
                                            // 5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
                                            // 10 = R, 11 = R2, 12 = R3, 13 = Time Trial
    int8            m_trackId;              // -1 for unknown, see appendix
    uint8           m_formula;              // Formula - 0 = F1 Modern, 1 = F1 Classic, 2 = F2
                                            // 3 = F1 Generic, 4 = Beta, 5 = Supercars
                                            // 6 = Esports, 7 = F2 2021
    uint16          m_sessionTimeLeft;      // Time left in session in seconds
    uint16          m_sessionDuration;      // Session duration in seconds
    uint8           m_pitSpeedLimit;        // Pit speed limit in km/h
    uint8           m_gamePaused;           // Whether the game is paused – network game only
    uint8           m_isSpectating;         // Whether the player is spectating
    uint8           m_spectatorCarIndex;    // Index of the car being spectated
    uint8           m_sliProNativeSupport;  // SLI Pro support 0 = inactive, 1 = active
    uint8           m_numMarshalZones;      // Number of marshal zones to follow
    MarshalZone     m_marshalZones[21];     // List of marshal zones – max 21
    uint8           m_safetyCarStatus;      // 0 = no safety car, 1 = full, 2 = virtual, 3 = formation lap
    uint8           m_networkGame;          // 0 = offline, 1 = online
    uint8           m_numWeatherForecastSamples; // Number of weather samples to follow
    WeatherForecastSample m_weatherForecastSamples[56];  // Array of weather forecast samples
    uint8           m_forecastAccuracy;     // 0 = Perfect, 1 = Approximate
    uint8           m_aiDifficulty;         // AI Difficulty rating – 0-110
    uint32          m_seasonLinkIdentifier; // Identifier for season - persists across saves
    uint32          m_weekendLinkIdentifier; // Identifier for weekend - persists across saves
    uint32          m_sessionLinkIdentifier; // Identifier for session - persists across saves
    uint8           m_pitStopWindowIdealLap;    // Ideal lap to pit on for current strategy (player)
    uint8           m_pitStopWindowLatestLap;   // Latest lap to pit on for current strategy (player)
    uint8           m_pitStopRejoinPosition;    // Predicted position to rejoin at (player)
    uint8           m_steeringAssist;           // 0 = off, 1 = on
    uint8           m_brakingAssist;            // 0 = off, 1 = low, 2 = medium, 3 = high
    uint8           m_gearboxAssist;            // 1 = manual, 2 = manual & suggested gear, 3 = auto
    uint8           m_pitAssist;                // 0 = off, 1 = on
    uint8           m_pitReleaseAssist;         // 0 = off, 1 = on
    uint8           m_ERSAssist;                // 0 = off, 1 = on
    uint8           m_DRSAssist;                // 0 = off, 1 = on
    uint8           m_dynamicRacingLine;        // 0 = off, 1 = corners only, 2 = full
    uint8           m_dynamicRacingLineType;    // 0 = 2D, 1 = 3D
    uint8           m_gameMode;                 // Game mode ID - see appendix
    uint8           m_ruleSet;                  // Ruleset - see appendix
    uint32          m_timeOfDay;                // Local time of day - minutes since midnight
    uint8           m_sessionLength;            // 0 = None, 2 = Very Short, 3 = Short, 4 = Medium
                                                // 5 = Medium Long, 6 = Long, 7 = Full
    uint8           m_speedUnitsLeadPlayer;     // 0 = MPH, 1 = KPH
    uint8           m_temperatureUnitsLeadPlayer; // 0 = Celsius, 1 = Fahrenheit
    uint8           m_speedUnitsSecondaryPlayer;  // 0 = MPH, 1 = KPH
    uint8           m_temperatureUnitsSecondaryPlayer; // 0 = Celsius, 1 = Fahrenheit
    uint8           m_numSafetyCarPeriods;       // Number of safety cars called during session
    uint8           m_numVirtualSafetyCarPeriods; // Number of virtual safety cars called
    uint8           m_numRedFlagPeriods;         // Number of red flags called during session
};
```

---

### Lap Data Packet

The lap data packet gives details of all the cars in the session.

- **Frequency**: Rate as specified in menus
- **Size**: 1131 bytes
- **Version**: 1

```c
struct LapData {
    uint32   m_lastLapTimeInMS;          // Last lap time in milliseconds
    uint32   m_currentLapTimeInMS;       // Current time around the lap in milliseconds
    uint16   m_sector1TimeInMS;          // Sector 1 time in milliseconds
    uint8    m_sector1TimeMinutes;       // Sector 1 whole minute part
    uint16   m_sector2TimeInMS;          // Sector 2 time in milliseconds
    uint8    m_sector2TimeMinutes;       // Sector 2 whole minute part
    uint16   m_deltaToCarInFrontInMS;    // Time delta to car in front in milliseconds
    uint16   m_deltaToRaceLeaderInMS;    // Time delta to race leader in milliseconds
    float    m_lapDistance;              // Distance vehicle is around current lap in meters
    float    m_totalDistance;            // Total distance traveled in session in meters
    float    m_safetyCarDelta;           // Delta in seconds for safety car
    uint8    m_carPosition;              // Car race position
    uint8    m_currentLapNum;            // Current lap number
    uint8    m_pitStatus;                // 0 = none, 1 = pitting, 2 = in pit area
    uint8    m_numPitStops;              // Number of pit stops taken in this race
    uint8    m_sector;                   // 0 = sector1, 1 = sector2, 2 = sector3
    uint8    m_currentLapInvalid;        // 0 = valid, 1 = invalid
    uint8    m

_penalties;                // Accumulated time penalties in seconds to be added
    uint8    m_totalWarnings;            // Accumulated number of warnings issued
    uint8    m_cornerCuttingWarnings;    // Accumulated number of corner cutting warnings issued
    uint8    m_numUnservedDriveThroughPens;  // Num drive through penalties left to serve
    uint8    m_numUnservedStopGoPens;       // Num stop go penalties left to serve
    uint8    m_gridPosition;             // Grid position the vehicle started the race in
    uint8    m_driverStatus;             // 0 = in garage, 1 = flying lap, 2 = in lap
                                         // 3 = out lap, 4 = on track
    uint8    m_resultStatus;             // 0 = invalid, 1 = inactive, 2 = active
                                         // 3 = finished, 4 = did not finish
                                         // 5 = disqualified, 6 = not classified, 7 = retired
    uint8    m_pitLaneTimerActive;       // Pit lane timing 0 = inactive, 1 = active
    uint16   m_pitLaneTimeInLaneInMS;    // If active the current time spent in the pit lane in ms
    uint16   m_pitStopTimerInMS;         // Time of the actual pit stop in ms
    uint8    m_pitStopShouldServePen;    // Whether the car should serve a penalty at this stop
};
```

```c
struct PacketLapData {
    PacketHeader    m_header;              // Header
    LapData         m_lapData[22];         // Lap data for all cars on track
    uint8           m_timeTrialPBCarIdx;   // Index of Personal Best car in time trial (255 if invalid)
    uint8           m_timeTrialRivalCarIdx; // Index of Rival car in time trial (255 if invalid)
};
```
---

### Event Packet

This packet gives details of events that happen during the course of a session.

- **Frequency**: When the event occurs
- **Size**: 45 bytes
- **Version**: 1

```c
union EventDataDetails {
    struct {
        uint8   vehicleIdx;   // Vehicle index of car achieving fastest lap
        float   lapTime;      // Lap time is in seconds
    } FastestLap;

    struct {
        uint8   vehicleIdx;   // Vehicle index of car retiring
    } Retirement;

    struct {
        uint8   vehicleIdx;   // Vehicle index of team mate
    } TeamMateInPits;

    struct {
        uint8   vehicleIdx;   // Vehicle index of the race winner
    } RaceWinner;

    struct {
        uint8   penaltyType;        // Penalty type – see Appendices
        uint8   infringementType;   // Infringement type – see Appendices
        uint8   vehicleIdx;         // Vehicle index of the car the penalty is applied to
        uint8   otherVehicleIdx;    // Vehicle index of the other car involved
        uint8   time;               // Time gained or time spent doing action in seconds
        uint8   lapNum;             // Lap the penalty occurred on
        uint8   placesGained;       // Number of places gained by this
    } Penalty;

    struct {
        uint8   vehicleIdx;               // Vehicle index of the vehicle triggering speed trap
        float   speed;                    // Top speed achieved in kilometers per hour
        uint8   isOverallFastestInSession; // Overall fastest speed in session = 1 otherwise 0
        uint8   isDriverFastestInSession;  // Fastest speed for driver in session = 1 otherwise 0
        uint8   fastestVehicleIdxInSession;// Vehicle index of the vehicle that is the fastest
        float   fastestSpeedInSession;     // Speed of the vehicle that is the fastest
    } SpeedTrap;

    struct {
        uint8   numLights;   // Number of lights showing
    } StartLights;

    struct {
        uint8   vehicleIdx;  // Vehicle index of the vehicle serving drive-through penalty
    } DriveThroughPenaltyServed;

    struct {
        uint8   vehicleIdx;  // Vehicle index of the vehicle serving stop-go penalty
    } StopGoPenaltyServed;

    struct {
        uint32  flashbackFrameIdentifier;  // Frame identifier flashed back to
        float   flashbackSessionTime;      // Session time flashed back to
    } Flashback;

    struct {
        uint32  buttonStatus;  // Bit flags specifying which buttons are being pressed
    } Buttons;

    struct {
        uint8   overtakingVehicleIdx;     // Vehicle index of the vehicle overtaking
        uint8   beingOvertakenVehicleIdx; // Vehicle index of the vehicle being overtaken
    } Overtake;
};
```

```c
struct PacketEventData {
    PacketHeader     m_header;                // Header
    uint8            m_eventStringCode[4];    // Event string code (see below)
    EventDataDetails m_eventDetails;          // Event details (interpret differently for each type)
};
```

**Event String Codes**

| Event            | Code  | Description                                                 |
|------------------|-------|-------------------------------------------------------------|
| Session Started  | SSTA  | Sent when the session starts                                 |
| Session Ended    | SEND  | Sent when the session ends                                   |
| Fastest Lap      | FTLP  | When a driver achieves the fastest lap                       |
| Retirement       | RTMT  | When a driver retires                                        |
| DRS enabled      | DRSE  | Race control has enabled DRS                                 |
| DRS disabled     | DRSD  | Race control has disabled DRS                                |
| Team mate in pits| TMPT  | Your team mate has entered the pits                          |
| Chequered flag   | CHQF  | The chequered flag has been waved                            |
| Race Winner      | RCWN  | The race winner is announced                                 |
| Penalty Issued   | PENA  | A penalty has been issued – details in event                 |
| Speed Trap       | SPTP  | Speed trap has been triggered by fastest speed               |
| Start lights     | STLG  | Start lights – number shown                                  |
| Lights out       | LGOT  | Lights out                                                   |
| Drive through    | DTSV  | Drive through penalty served                                 |
| Stop go          | SGSV  | Stop-go penalty served                                       |
| Flashback        | FLBK  | Flashback activated                                          |
| Button status    | BUTN  | Button status changed                                        |
| Red Flag         | RDFL  | Red flag shown                                               |
| Overtake         | OVTK  | Overtake occurred                                            |

---

### Participants Packet

This is a list of participants in the race. If the vehicle is controlled by AI, then the name will be the driver name. In multiplayer, the names will be the Steam ID on PC or the LAN name if appropriate. The array should be indexed by vehicle index.

- **Frequency**: Every 5 seconds
- **Size**: 1306 bytes
- **Version**: 1

```c
struct ParticipantData {
    uint8     m_aiControlled;   // Whether the vehicle is AI (1) or Human (0) controlled
    uint8     m_driverId;       // Driver ID - see appendix, 255 if network human
    uint8     m_networkId;      // Network ID – unique identifier for network players
    uint8     m_teamId;         // Team ID - see appendix
    uint8     m_myTeam;         // My team flag – 1 = My Team, 0 = otherwise
    uint8     m_raceNumber;     // Race number of the car
    uint8     m_nationality;    // Nationality of the driver
    char      m_name[48];       // Name of participant in UTF-8 format – null terminated
                                // Will be truncated with … (U+2026) if too long
    uint8     m_yourTelemetry;  // The player's UDP setting 0 = restricted, 1 = public
    uint8     m_showOnlineNames; // The player's show online names setting, 0 = off, 1 = on
    uint8     m_platform;       // 1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
};
```

```c
struct PacketParticipantsData {
    PacketHeader    m_header;           // Header
    uint8           m_numActiveCars;    // Number of active cars in the data – should match number of cars on HUD
    ParticipantData m_participants[22]; // Data for all participants
};
```

---

### Car Setups Packet

This packet details the car setups for each vehicle in the session. In multiplayer games, other player cars will appear as blank, and spectators cannot see any car setups.

- **Frequency**: 2 per second
- **Size**: 1107 bytes
- **Version**: 1

```c
struct CarSetupData {
    uint8     m_frontWing;               // Front wing aero
    uint8     m_rearWing;                // Rear wing aero
    uint8     m_onThrottle;              // Differential adjustment on throttle (percentage)
    uint8     m_offThrottle;             // Differential adjustment off throttle (percentage)
    float     m_frontCamber;             // Front camber angle (suspension geometry)
    float     m_rearCamber;              // Rear camber angle (suspension geometry)
    float     m_frontToe;                // Front toe angle (suspension geometry)
    float     m_rearToe;                 // Rear toe angle (suspension geometry)
    uint8     m_frontSuspension;         // Front suspension
    uint8     m_rearSuspension;          // Rear suspension
    uint8     m_frontAntiRollBar;        // Front anti-roll bar
    uint8     m_rearAntiRollBar;         // Rear anti-roll bar
    uint8     m_frontSuspensionHeight;   // Front ride height
    uint8     m_rearSuspensionHeight;    // Rear ride height
    uint8     m_brakePressure;           // Brake pressure (percentage)
    uint8     m_brakeBias;               // Brake bias (percentage)
    float     m_rearLeftTyrePressure;    // Rear left tyre pressure (PSI)
    float     m_rearRightTyrePressure;   // Rear right tyre pressure (PSI)
    float     m_frontLeftTyrePressure;   // Front left tyre pressure (PSI)
    float     m_frontRightTyrePressure;  // Front right tyre pressure (PSI)
    uint8     m_ballast;                 // Ballast
    float     m_fuelLoad;                // Fuel load
};
```

```c
struct PacketCarSetupData {
    PacketHeader    m_header;             // Header
    CarSetupData    m_carSetups[22];      // Data for all cars on track
};
```
---

### Car Telemetry Packet

This packet details telemetry for all the cars in the race. It provides various values that would be recorded on the car, such as speed, throttle application, DRS status, etc.

- **Frequency**: Rate as specified in menus
- **Size**: 1352 bytes
- **Version**: 1

```c
struct CarTelemetryData {
    uint16    m_speed;                    // Speed of car in kilometers per hour
    float     m_throttle;                 // Amount of throttle applied (0.0 to 1.0)
    float     m_steer;                    // Steering (-1.0 (full left lock) to 1.0 (full right lock))
    float     m_brake;                    // Amount of brake applied (0.0 to 1.0)
    uint8     m_clutch;                   // Amount of clutch applied (0 to 100)
    int8      m_gear;                     // Gear selected (1-8, N=0, R=-1)
    uint16    m_engineRPM;                // Engine RPM
    uint8     m_drs;                      // 0 = off, 1 = on
    uint8     m_revLightsPercent;         // Rev lights indicator (percentage)
    uint16    m_revLightsBitValue;        // Rev lights (bit 0 = leftmost LED, bit 14 = rightmost LED)
    uint16    m_brakesTemperature[4];     // Brakes temperature (Celsius)
    uint8     m_tyresSurfaceTemperature[4]; // Tyres surface temperature (Celsius)
    uint8     m_tyresInnerTemperature[4]; // Tyres inner temperature (Celsius)
    uint16    m_engineTemperature;        // Engine temperature (Celsius)
    float     m_tyresPressure[4];         // Tyres pressure (PSI)
    uint8     m_surfaceType[4];           // Driving surface (see Appendices)
};
```

```c
struct PacketCarTelemetryData {
    PacketHeader       m_header;                // Header
    CarTelemetryData   m_carTelemetryData[22];  // Telemetry data for all cars
    uint8             m_mfdPanelIndex;          // Index of MFD panel open - 255 = MFD closed
                                                // Single player race – 0 = Car setup, 1 = Pits
                                                // 2 = Damage, 3 = Engine, 4 = Temperatures
    uint8             m_mfdPanelIndexSecondaryPlayer;  // See above
    int8              m_suggestedGear;          // Suggested gear for the player (1-8), 0 = none
};
```

---

### Car Status Packet

This packet details the status of all the cars in the race, such as fuel, ERS, and engine health.

- **Frequency**: Rate as specified in menus
- **Size**: 1239 bytes
- **Version**: 1

```c
struct CarStatusData {
    uint8    m_tractionControl;            // Traction control level (0 = off, 1 = medium, 2 = full)
    uint8    m_antiLockBrakes;             // ABS (0 = off, 1 = on)
    uint8    m_fuelMix;                    // Fuel mix (0 = lean, 1 = standard, 2 = rich, 3 = max)
    uint8    m_frontBrakeBias;             // Front brake bias (percentage)
    uint8    m_pitLimiterStatus;           // Pit limiter status (0 = off, 1 = on)
    float    m_fuelInTank;                 // Current fuel mass
    float    m_fuelCapacity;               // Fuel capacity
    float    m_fuelRemainingLaps;          // Fuel remaining in laps (value on MFD)
    uint16   m_maxRPM;                     // Car's max RPM point (rev limiter)
    uint16   m_idleRPM;                    // Car's idle RPM
    uint8    m_maxGears;                   // Maximum number of gears
    uint8    m_drsAllowed;                 // DRS allowed (0 = not allowed, 1 = allowed)
    uint16   m_drsActivationDistance;      // DRS activation distance in meters (0 = DRS not available)
    uint8    m_actualTyreCompound;         // F1 Modern: actual tyre compound, see Appendices
    uint8    m_visualTyreCompound;         // Visual tyre compound (could differ from actual)
    uint8    m_tyresAgeLaps;               // Age in laps of the current set of tyres
    int8     m_vehicleFiaFlags;            // -1 = invalid, 0 = none, 1 = green, 2 = blue, 3 = yellow
    float    m_enginePowerICE;             // Engine power output of ICE (W)
    float    m_enginePowerMGUK;            // Engine power output of MGU-K (W)
    float    m_ersStoreEnergy;             // ERS energy store in Joules
    uint8    m_ersDeployMode;              // ERS deployment mode (0 = none, 1 = medium, 2 = hotlap, 3 = overtake)
    float    m_ersHarvestedThisLapMGUK;    // ERS energy harvested this lap by MGU-K
    float    m_ersHarvestedThisLapMGUH;    // ERS energy harvested this lap by MGU-H
    float    m_ersDeployedThisLap;         // ERS energy deployed this lap
    uint8    m_networkPaused;              // Whether the car is paused in a network game
};
```

```c
struct PacketCarStatusData {
    PacketHeader     m_header;          // Header
    CarStatusData    m_carStatusData[22]; // Status data for all cars
};
```

---

### Final Classification Packet

This packet details the final classification at the end of the race, matching the post-race results screen. This is especially useful for multiplayer games, where it might not be possible to send lap times on the final frame due to network delay.

- **Frequency**: Once at the end of a race
- **Size**: 1020 bytes
- **Version**: 1

```c
struct FinalClassificationData {
    uint8     m_position;              // Finishing position
    uint8     m_numLaps;               // Number of laps completed
    uint8     m_gridPosition;          // Grid position of the car
    uint8     m_points;                // Number of points scored
    uint8     m_numPitStops;           // Number of pit stops made
    uint8     m_resultStatus;          // Result status - 0 = invalid, 1 = inactive, 2 = active
                                       // 3 = finished, 4 = did not finish, 5 = disqualified
                                       // 6 = not classified, 7 = retired
    uint32    m_bestLapTimeInMS;       // Best lap time of the session in milliseconds
    double    m_totalRaceTime;         // Total race time in seconds without penalties
    uint8     m_penaltiesTime;         // Total penalties accumulated in seconds
    uint8     m_numPenalties;          // Number of penalties applied to this driver
    uint8     m_numTyreStints;         // Number of tyre stints
    uint8     m_tyreStintsActual[8];   // Actual tyres used by this driver
    uint8     m_tyreStintsVisual[8];   // Visual tyres used by this driver
    uint8     m_tyreStintsEndLaps[8];  // The lap number stints end on
};
```

```c
struct PacketFinalClassificationData {
    PacketHeader          m_header;                  // Header
    uint8                m_numCars;                  // Number of cars in the final classification
    FinalClassificationData m_classificationData[22]; // Final classification data for all cars
};
```

---

### Lobby Info Packet

This packet details the players currently in a multiplayer lobby, including each player's selected car, any AI involved in the game, and the ready status of each participant.

- **Frequency**: Two every second when in the lobby
- **Size**: 1218 bytes
- **Version**: 1

```c
struct LobbyInfoData {
    uint8     m_aiControlled;      // Whether the vehicle is AI (1) or Human (0) controlled
    uint8     m_teamId;            // Team ID - see appendix (255 if no team selected)
    uint8     m_nationality;       // Nationality of the driver
    uint8     m_platform;          // Platform (1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown)
    char      m_name[48];          // Name of participant in UTF-8 format – null terminated
                                   // Will be truncated with ... (U+2026) if too long
    uint8     m_carNumber;         // Car number of the player
    uint8     m_readyStatus;       // 0 = not ready, 1 = ready, 2 = spectating
};
```

```c
struct PacketLobbyInfoData {
    PacketHeader    m_header;                 // Header
    uint8           m_numPlayers;             // Number of players in the lobby data
    LobbyInfoData   m_lobbyPlayers[22];       // Lobby info for all players
};
```
---

### Car Damage Packet

This packet details car damage parameters for all the cars in the race.

- **Frequency**: 10 per second
- **Size**: 953 bytes
- **Version**: 1

```c
struct CarDamageData {
    float     m_tyresWear[4];               // Tyre wear (percentage)
    uint8     m_tyresDamage[4];             // Tyre damage (percentage)
    uint8     m_brakesDamage[4];            // Brakes damage (percentage)
    uint8     m_frontLeftWingDamage;        // Front left wing damage (percentage)
    uint8     m_frontRightWingDamage;       // Front right wing damage (percentage)
    uint8     m_rearWingDamage;             // Rear wing damage (percentage)
    uint8     m_floorDamage;                // Floor damage (percentage)
    uint8     m_diffuserDamage;             // Diffuser damage (percentage)
    uint8     m_sidepodDamage;              // Sidepod damage (percentage)
    uint8     m_drsFault;                   // DRS fault indicator (0 = OK, 1 = fault)
    uint8     m_ersFault;                   // ERS fault indicator (0 = OK, 1 = fault)
    uint8     m_gearBoxDamage;              // Gearbox damage (percentage)
    uint8     m_engineDamage;               // Engine damage (percentage)
    uint8     m_engineMGUHWear;             // MGU-H wear (percentage)
    uint8     m_engineESWear;               // Energy store wear (percentage)
    uint8     m_engineCEWear;               // CE wear (percentage)
    uint8     m_engineICEWear;              // ICE wear (percentage)
    uint8     m_engineMGUKWear;             // MGU-K wear (percentage)
    uint8     m_engineTCWear;               // Turbocharger wear (percentage)
    uint8     m_engineBlown;                // Engine blown (0 = OK, 1 = blown)
    uint8     m_engineSeized;               // Engine seized (0 = OK, 1 = seized)
};
```

```c
struct PacketCarDamageData {
    PacketHeader    m_header;             // Header
    CarDamageData   m_carDamageData[22];  // Damage data for all cars
};
```

---

### Session History Packet

This packet contains lap times and tyre usage for the session. It works slightly differently from other packets. To reduce CPU and bandwidth, each packet relates to a specific vehicle and is sent every 1/20 s, cycling through cars. Therefore, in a 20-car race, you should receive an update for each vehicle at least once per second.

- **Frequency**: 20 per second but cycling through cars
- **Size**: 1460 bytes
- **Version**: 1

```c
struct LapHistoryData {
    uint32    m_lapTimeInMS;          // Lap time in milliseconds
    uint16    m_sector1TimeInMS;      // Sector 1 time in milliseconds
    uint8     m_sector1TimeMinutes;   // Sector 1 whole minute part
    uint16    m_sector2TimeInMS;      // Sector 2 time in milliseconds
    uint8     m_sector2TimeMinutes;   // Sector 2 whole minute part
    uint16    m_sector3TimeInMS;      // Sector 3 time in milliseconds
    uint8     m_sector3TimeMinutes;   // Sector 3 whole minute part
    uint8     m_lapValidBitFlags;     // Lap validity flags (0x01 = lap valid, etc.)
};
```

```c
struct TyreStintHistoryData {
    uint8     m_endLap;               // Lap the tyre usage ends on (255 if current tyre)
    uint8     m_tyreActualCompound;   // Actual tyres used
    uint8     m_tyreVisualCompound;   // Visual tyres used
};
```

```c
struct PacketSessionHistoryData {
    PacketHeader             m_header;              // Header
    uint8                   m_carIdx;              // Index of the car this lap data relates to
    uint8                   m_numLaps;             // Number of laps in the data
    uint8                   m_numTyreStints;       // Number of tyre stints in the data
    uint8                   m_bestLapTimeLapNum;   // Lap with best lap time
    uint8                   m_bestSector1LapNum;   // Lap with best sector 1 time
    uint8                   m_bestSector2LapNum;   // Lap with best sector 2 time
    uint8                   m_bestSector3LapNum;   // Lap with best sector 3 time
    LapHistoryData          m_lapHistoryData[100]; // Lap history data for 100 laps max
    TyreStintHistoryData    m_tyreStintsHistoryData[8]; // Tyre stint history data
};
```

---

### Tyre Sets Packet

This packet gives more in-depth details about tyre sets assigned to a vehicle during the session.

- **Frequency**: 20 per second but cycling through cars
- **Size**: 231 bytes
- **Version**: 1

```c
struct TyreSetData {
    uint8    m_actualTyreCompound;    // Actual tyre compound used
    uint8    m_visualTyreCompound;    // Visual tyre compound used
    uint8    m_wear;                  // Tyre wear (percentage)
    uint8    m_available;             // Whether this set is currently available
    uint8    m_recommendedSession;    // Recommended session for tyre set
    uint8    m_lifeSpan;              // Laps left in this tyre set
    uint8    m_usableLife;            // Max laps recommended for this compound
    int16    m_lapDeltaTime;          // Lap delta time compared to fitted set (milliseconds)
    uint8    m_fitted;                // Whether the set is fitted (0 = not fitted, 1 = fitted)
};
```

```c
struct PacketTyreSetsData {
    PacketHeader    m_header;             // Header
    uint8           m_carIdx;             // Index of the car this data relates to
    TyreSetData     m_tyreSetData[20];    // Data for 13 (dry) + 7 (wet) tyres
    uint8           m_fittedIdx;          // Index into array of fitted tyre
};
```

---

### Motion Ex Packet

The motion packet gives extended data for the car being driven with the goal of driving a motion platform setup.

- **Frequency**: Rate as specified in menus
- **Size**: 217 bytes
- **Version**: 1

```c
struct PacketMotionExData {
    PacketHeader    m_header;                  // Header
    // Extra player car ONLY data
    float           m_suspensionPosition[4];   // Suspension position (RL, RR, FL, FR)
    float           m_suspensionVelocity[4];   // Suspension velocity (RL, RR, FL, FR)
    float           m_suspensionAcceleration[4]; // Suspension acceleration (RL, RR, FL, FR)
    float           m_wheelSpeed[4];           // Wheel speed (RL, RR, FL, FR)
    float           m_wheelSlipRatio[4];       // Slip ratio for each wheel
    float           m_wheelSlipAngle[4];       // Slip angle for each wheel
    float           m_wheelLatForce[4];        // Lateral force for each wheel
    float           m_wheelLongForce[4];       // Longitudinal force for each wheel
    float           m_heightOfCOGAboveGround;  // Height of center of gravity above ground
    float           m_localVelocityX;          // Local velocity in X axis (m/s)
    float           m_localVelocityY;          // Local velocity in Y axis (m/s)
    float           m_localVelocityZ;          // Local velocity in Z axis (m/s)
    float           m_angularVelocityX;        // Angular velocity X component (radians/s)
    float           m_angularVelocityY;        // Angular velocity Y component
    float           m_angularVelocityZ;        // Angular velocity Z component
    float           m_angularAccelerationX;    // Angular acceleration X component (radians/s²)
    float           m_angularAccelerationY;    // Angular acceleration Y component
    float           m_angularAccelerationZ;    // Angular acceleration Z component
    float           m_frontWheelsAngle;        // Current front wheels angle (radians)
    float           m_wheelVertForce[4];       // Vertical force for each wheel
};
```
---

## Restricted Data (Your Telemetry Setting)

There is some data in the UDP that you may not want other players to see in multiplayer games. This is controlled by the **"Your Telemetry"** setting in the telemetry options.

### Options:
- **Restricted** (Default) – Other players viewing the UDP data will not see values for your car.
- **Public** – All other players can see all the data for your car.
- **Show Online ID** – This additional option allows other players to view your online ID / gamertag in their UDP output.

### Data items that are set to zero if "Your Telemetry" is set to "Restricted":

#### **Car Status Packet:**
- `m_fuelInTank`
- `m_fuelCapacity`
- `m_fuelMix`
- `m_fuelRemainingLaps`
- `m_frontBrakeBias`
- `m_ersDeployMode`
- `m_ersStoreEnergy`
- `m_ersDeployedThisLap`
- `m_ersHarvestedThisLapMGUK`
- `m_ersHarvestedThisLapMGUH`
- `m_enginePowerICE`
- `m_enginePowerMGUK`

#### **Car Damage Packet:**
- `m_frontLeftWingDamage`
- `m_frontRightWingDamage`
- `m_rearWingDamage`
- `m_floorDamage`
- `m_diffuserDamage`
- `m_sidepodDamage`
- `m_engineDamage`
- `m_gearBoxDamage`
- `m_tyresWear` (All four wheels)
- `m_tyresDamage` (All four wheels)
- `m_brakesDamage` (All four wheels)
- `m_drsFault`
- `m_engineMGUHWear`
- `m_engineESWear`
- `m_engineCEWear`
- `m_engineICEWear`
- `m_engineMGUKWear`
- `m_engineTCWear`

#### **Tyre Set Packet:**
All data within this packet for the player car.

To allow other players to view your online ID in their UDP output during an online session, you must enable the **"Show Online ID / Gamertags"** option.

---

## FAQS

### How do I enable the UDP Telemetry Output?

In F1 23, the UDP telemetry output is controlled via the in-game menus. To enable this, enter the options menu from the main menu (triangle/Y), then enter the settings menu. The UDP option will be at the bottom of the list.

- From there, you will be able to:
  - Enable / disable the UDP output.
  - Configure the IP address and port for the receiving application.
  - Toggle broadcast mode and set the send rate.

**Broadcast Mode** transmits the data across the network subnet, allowing multiple devices on the same subnet to receive the information. In broadcast mode, it is not necessary to set a target IP address, just a target port for applications to listen on.

### Advanced PC Users:

You can additionally edit the game’s configuration XML file to configure UDP output. The file is located here (after an initial boot of the game):

```
...\Documents\My Games\<game_folder>\hardwaresettings\hardware_settings_config.xml
```

You should see the following tag:

```xml
<motion>
    ...
    <udp enabled="false" broadcast="false" ip="127.0.0.1" port="20777" sendRate="20" format="2023" yourTelemetry="restricted" onlineNames="off" />
    ...
</motion>
```

Here you can set the values manually. Any changes made within the game while it is running will overwrite changes made manually. Note that the `enabled` flag now represents a state.

---

### What has changed since last year?

F1® 23 sees the following changes to the UDP specification:
- Added game year to packet header – apps can identify which F1 game the data is coming from.
- Temperature and speed units choice for players sent in session packet.
- Platform of players added to lobby info and participants packets.
- Added flag to indicate whether a player has their “Show online names” flag set in participants packet.
- Added whole minute part to sector times in lap data and session history packets.
- Damage packet now updates at 10/s.
- Separated corner-cutting warnings in the lap data packet.
- Added a new tyre sets packet to provide more details about tyre sets for each car.
- Added time deltas for cars in the lap data packet.
- Added overall frame identifier to packet header to help with handling flashbacks.
- Added Red Flag event.
- Added Safety car, VSC, and Red Flag counts to session data.
- Added more physics data to the motion packet.
- Added Overtake event.
- Added power output readings for the engine.
- Added C0 tyre type.
- Added a new Motion Ex packet and moved player car settings from Motion packet to stop it from getting too large, added vertical wheel forces.

---

### What is the order of the wheel arrays?

All wheel arrays are in the following order:
1. Rear Left (RL)
2. Rear Right (RR)
3. Front Left (FL)
4. Front Right (FR)

---

### Do the vehicle indices change?

During a session, each car is assigned a vehicle index. This will not change throughout the session, and all arrays use this vehicle index to dereference the correct piece of data.

---

### What coordinate systems are used?

A visual representation of the coordinate system used with the F1 telemetry data is provided in the document (not displayed here).

---

### What encoding format is used?

All values are encoded using **Little Endian** format.

---

### Are the data structures packed?

Yes, all data is packed. No padding is used.

---

### How many cars are in the data structures?

The maximum number of cars in the data structures is 22. You should check the data item called `m_numActiveCars` in the participants packet to see how many cars are active in the race. However, to know if a car is actively providing data, check the individual result status of each car in the lap data packet.

---

### How often are updated packets sent?

For packets updated at the "Rate as specified in the menus," you can be guaranteed that all relevant packets will be sent together on the same frame. However, the reliability of the network affects whether they are received correctly.

---

### Will my old app still work with F1 23?

F1 23 supports the previous two UDP formats, so most older apps using these formats should work with little or no change. You can select "UDP Format" to either "2022" or "2021" in the options menu.

---

### How do I enable D-BOX output?

D-BOX output is supported on the PC platform. You can enable it via the in-game menu: **Game Options -> Settings -> UDP Telemetry Settings -> D-BOX**.

---

### How can I disable in-game support for LED devices?

The F1 game has native support for some LED devices, such as the Leo Bodnar SLI Pro and Fanatec steering wheels. You can disable the native support by editing the `hardware_settings_config.xml` file in the following location:

```
...\Documents\My Games\<game_folder>\hardwaresettings\hardware_settings_config.xml
```

Set the following flags to `false` to disable LED output:

```xml
<led_display fanatecNativeSupport="true" sliProNativeSupport="true" />
```

---

### Can I configure the UDP output using an XML file?

Yes, PC users can manually edit the configuration XML file to set up UDP output:

```
...\Documents\My Games\<game_folder>\hardwaresettings\hardware_settings_config.xml
```

Here is the relevant XML tag:

```xml
<motion>
    ...
    <udp enabled="false" broadcast="false" ip="127.0.0.1" port="20777" sendRate="20" format="2023" yourTelemetry="restricted" onlineNames="off" />
    ...
</motion>
```

---

## Appendices

Here are the values used for some of the parameters in the UDP data output.

---

### Team IDs

| ID  | Team                              |
|-----|-----------------------------------|
| 0   | Mercedes                          |
| 1   | Ferrari                           |
| 2   | Red Bull Racing                   |
| 3   | Williams                          |
| 4   | Aston Martin                      |
| 5   | McLaren                           |
| 6   | Alpine                            |
| 7   | Alfa Romeo                        |
| 8   | Haas                              |
| 9   | Pirelli                           |
| 10  | Prema ‘21                         |
| 11  | Uni-Virtuosi ‘21                  |
| 12  | Carlin ‘21                        |
| 13  | Hitech ‘21                        |
| 14  | Art GP ‘21                        |
| 15  | Campos ‘21                        |
| 16  | Van Amersfoort ‘21               |
| 17  | Trident ‘21                       |
| 18  | Mercedes '22                      |
| 19  | Ferrari '22                       |
| 20  | Red Bull Racing '22               |
| 21  | Williams '22                      |

---

### Driver IDs

| ID  | Driver                             |
|-----|------------------------------------|
| 0   | Lewis Hamilton                     |
| 1   | Valtteri Bottas                   |
| 2   | Sebastian Vettel                   |
| 3   | Charles Leclerc                    |
| 4   | Carlos Sainz                       |
| 5   | Lando Norris                       |
| 6   | Daniel Ricciardo                   |
| 7   | Fernando Alonso                    |
| 8   | Esteban Ocon                       |
| 9   | Pierre Gasly                       |
| 10  | Lance Stroll                      |
| 11  | Sebastian Vettel                   |
| 12  | Kevin Magnussen                    |
| 13  | Nico Hülkenberg                   |
| 14  | Yuki Tsunoda                       |
| 15  | Nyck de Vries                      |
| 16  | Oscar Piastri                     |
| 17  | Logan Sargeant                     |
| 18  | Alex Albon                         |
| 19  | Zhou Guanyu                        |
| 20  | George Russell                     |

---

### Track IDs

| ID  | Track                             |
|-----|------------------------------------|
| 0   | Melbourne Grand Prix              |
| 1   | Saudi Arabian Grand Prix          |
| 2   | Australian Grand Prix             |
| 3   | Italian Grand Prix                |
| 4   | Canadian Grand Prix               |
| 5   | Austrian Grand Prix               |
| 6   | British Grand Prix                |
| 7   | Hungarian Grand Prix              |
| 8   | Belgian Grand Prix                |
| 9   | Dutch Grand Prix                  |
| 10  | Singapore Grand Prix              |
| 11  | Japanese Grand Prix               |
| 12  | United States Grand Prix          |
| 13  | Mexican Grand Prix                |
| 14  | Brazilian Grand Prix              |
| 15  | Las Vegas Grand Prix              |
| 16  | Abu Dhabi Grand Prix              |

---

### Nationality IDs

| ID  | Nationality                      |
|-----|----------------------------------|
| 0   | British                          |
| 1   | German                           |
| 2   | French                           |
| 3   | Spanish                          |
| 4   | Italian                          |
| 5   | Dutch                            |
| 6   | Australian                       |
| 7   | Canadian                         |
| 8   | American                         |
| 9   | Japanese                         |
| 10  | Mexican                          |
| 11  | Thai                             |
| 12  | Brazilian                        |
| 13  | Belgian                          |
| 14  | Danish                           |
| 15  | Chinese                          |
| 16  | Indian                           |
| 17  | Indonesian                       |
| 18  | Russian                          |
| 19  | Saudi Arabian                    |

---

### Game Mode IDs

| ID  | Game Mode                       |
|-----|---------------------------------|
| 0   | Unknown                         |
| 1   | Career                          |
| 2   | My Team                         |
| 3   | Time Trial                      |
| 4   | Grand Prix                      |
| 5   | Online                          |
| 6   | Championships                   |
| 7   | Esports                         |

---

### Ruleset IDs

| ID  | Ruleset                         |
|-----|---------------------------------|
| 0   | Unknown                         |
| 1   | Formula 1                      |
| 2   | Formula 2                      |
| 3   | Formula 3                      |
| 4   | Formula E                      |
| 5   | Classic Cars                   |

---

### Encoding Format

All values are encoded using **Little Endian** format.

---

### Glossary of Terms

- **UDP**: User Datagram Protocol, a communication protocol used for real-time data transfer.
- **Telemetry**: Data collected from the car during a race to monitor performance and conditions.
- **DRS**: Drag Reduction System, a device that reduces aerodynamic drag to increase speed.
- **ERS**: Energy Recovery System, a system that recovers energy during braking and provides a power boost.
- **AI**: Artificial Intelligence, used to control non-player characters (NPCs) in the game.

---

### Legal Notice

This document and its contents are the property of EA and are provided under license for reference purposes only. No representations or warranties about the accuracy or reliability of the information for any specific purpose are made.

---