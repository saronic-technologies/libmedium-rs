use crate::units::{Error as UnitError, Raw, Result as UnitResult};
use std::borrow::Cow;
use std::fmt;

/// Struct that represents a pwm value between 0 and 255.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Pwm(u8);

impl Pwm {
    /// Create a new Pwm struct from a pwm value between 0 and 255.
    pub fn from_u8(u8: u8) -> Self {
        Self(u8)
    }

    /// Returns this struct's pwm value.
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Tries to create a new `Pwm` struct from a pwm value in percent.
    /// Returns an error if the given value is not between 0 and 100.
    pub fn try_from_percent(percent: impl Into<f64>) -> UnitResult<Self> {
        let percent = percent.into();

        if percent.is_nan() || percent < 0.0 || percent > 100.0 {
            return Err(UnitError::invalid_value(percent));
        }

        Ok(Pwm((percent * 2.55) as u8))
    }

    /// Returns this struct's pwm value in percent.
    pub fn as_percent(self) -> f64 {
        f64::from(self.0) / 2.55
    }
}

impl From<u8> for Pwm {
    fn from(value: u8) -> Pwm {
        Pwm::from_u8(value)
    }
}

impl From<Pwm> for u8 {
    fn from(value: Pwm) -> u8 {
        value.0
    }
}

impl Raw for Pwm {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.parse::<u8>()
            .map(Pwm::from_u8)
            .map_err(|_| UnitError::raw_conversion(raw))
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
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PwmEnable {
    FullSpeed,
    ManualControl,
    BiosControl,
}

impl Raw for PwmEnable {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        match raw {
            "0" => Ok(PwmEnable::FullSpeed),
            "1" => Ok(PwmEnable::ManualControl),
            _ => Ok(PwmEnable::BiosControl),
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

/// Enum that represents the modes by which a fan's speed can be regulated.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PwmMode {
    Dc,
    Pwm,
    Automatic,
}

impl Raw for PwmMode {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        match raw {
            "0" => Ok(PwmMode::Dc),
            "1" => Ok(PwmMode::Pwm),
            "2" => Ok(PwmMode::Automatic),
            raw => Err(UnitError::raw_conversion(raw)),
        }
    }

    fn to_raw(&self) -> Cow<str> {
        match self {
            PwmMode::Dc => Cow::from("0"),
            PwmMode::Pwm => Cow::from("1"),
            PwmMode::Automatic => Cow::from("2"),
        }
    }
}

impl Default for PwmMode {
    fn default() -> PwmMode {
        PwmMode::Automatic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Pwm::try_from_percent(-1.1).is_err());
        assert!(Pwm::try_from_percent(0.0).is_ok());
        assert!(Pwm::try_from_percent(50.0).is_ok());
        assert!(Pwm::try_from_percent(100.0).is_ok());
        assert!(Pwm::try_from_percent(100.001).is_err());
        assert!(Pwm::try_from_percent(f64::INFINITY).is_err());
        assert!(Pwm::try_from_percent(f64::NAN).is_err());
    }
}
