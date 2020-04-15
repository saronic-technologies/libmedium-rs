use crate::units::{Raw, RawError, RawSensorResult};
use std::borrow::Cow;
use std::fmt;

/// Struct that represents a pwm value between 0 and 255.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Pwm(u8);

impl Pwm {
    /// Construct a new Pwm struct from a pwm value between 0 and 255.
    pub fn from_u8(u8: u8) -> Self {
        Self(u8)
    }

    /// Returns this struct's pwm value between 0 and 255.
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Construct a new Pwm struct from a pwm value in percent.
    pub fn from_percent(percent: f64) -> Self {
        Self((percent * 2.55) as u8)
    }

    /// Returns this struct's pwm value in percent.
    pub fn as_percent(self) -> f64 {
        f64::from(self.0) / 2.55
    }
}

impl Raw for Pwm {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.parse::<u8>()
            .map(Pwm::from_u8)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }
}

impl fmt::Display for Pwm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

/// Enum that represents the control states a pwm can be in.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum PwmEnable {
    FullSpeed,
    ManualControl,
    BiosControl,
}

impl Raw for PwmEnable {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "0" => Ok(PwmEnable::FullSpeed),
            "1" => Ok(PwmEnable::ManualControl),
            "2" => Ok(PwmEnable::BiosControl),
            raw => Err(raw.into()),
        }
    }

    fn to_raw(&self) -> Cow<str> {
        match self {
            PwmEnable::FullSpeed => Cow::from("0"),
            PwmEnable::ManualControl => Cow::from("1"),
            PwmEnable::BiosControl => Cow::from("2"),
        }
    }
}

impl Default for PwmEnable {
    fn default() -> PwmEnable {
        PwmEnable::BiosControl
    }
}

/// Struct that represents the modes by which a fan's speed can be regulated.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum PwmMode {
    DC,
    PWM,
    Automatic,
}

impl Raw for PwmMode {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "0" => Ok(PwmMode::DC),
            "1" => Ok(PwmMode::PWM),
            "2" => Ok(PwmMode::Automatic),
            raw => Err(raw.into()),
        }
    }

    fn to_raw(&self) -> Cow<str> {
        match self {
            PwmMode::DC => Cow::from("0"),
            PwmMode::PWM => Cow::from("1"),
            PwmMode::Automatic => Cow::from("2"),
        }
    }
}

impl Default for PwmMode {
    fn default() -> PwmMode {
        PwmMode::Automatic
    }
}
