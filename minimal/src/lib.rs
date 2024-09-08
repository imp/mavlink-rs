#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    // FromPrimitive, ToPrimitive
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[repr(u8)]
pub enum MavState {
    #[default]
    Uninit = 0,
    Boot = 1,
    Calibraring = 2,
    Standby = 3,
    Active = 4,
    Critical = 5,
    Emergency = 6,
    Poweroff = 7,
    FlightTermination = 8,
}

impl MavState {
    pub const MAV_STATE_UNINIT: Self = Self::Uninit;
    pub const MAV_STATE_BOOT: Self = Self::Boot;
    pub const MAV_STATE_CALIBRATING: Self = Self::Calibraring;
    pub const MAV_STATE_STANDBY: Self = Self::Standby;
    pub const MAV_STATE_ACTIVE: Self = Self::Active;
    pub const MAV_STATE_CRITICAL: Self = Self::Critical;
    pub const MAV_STATE_EMERGENCY: Self = Self::Emergency;
    pub const MAV_STATE_POWEROFF: Self = Self::Poweroff;
    pub const MAV_STATE_FLIGHT_TERMINATION: Self = Self::FlightTermination;
    pub const DEFAULT: Self = Self::MAV_STATE_UNINIT;
}
