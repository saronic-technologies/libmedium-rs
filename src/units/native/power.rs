use crate::units::{Raw, RawError, RawSensorResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents electrical power.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Power(u32);

impl Power {
    /// Create a Power struct from a value measuring watts.
    pub fn from_watts(degrees: impl Into<f64>) -> Self {
        Self((degrees.into() * 1_000_000.0) as u32)
    }

    /// Create a Power struct from a value measuring microwatts.
    pub fn from_microwatts(microwatts: impl Into<u32>) -> Self {
        Self(microwatts.into())
    }

    /// Returns this struct's value as watts.
    pub fn as_watts(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }

    /// Returns this struct's value as microwatts.
    pub fn as_microwatts(self) -> u32 {
        self.0
    }
}

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Power::from_microwatts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_microwatts().to_string())
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}W", self.as_watts())
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Power(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Power {
    type Output = Self;

    fn mul(self, other: T) -> Power {
        Power(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Power {
    type Output = Self;

    fn div(self, other: T) -> Power {
        Power(self.0 / other.into())
    }
}
