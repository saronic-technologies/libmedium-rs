use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents used energy.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Energy(u32);

impl Energy {
    /// Create an Energy struct from a value measuring microjoules.
    pub fn from_micro_joules(micros: u32) -> Energy {
        Energy(micros)
    }

    /// Return this Energy's value in microjoules.
    pub fn as_micro_joules(self) -> u32 {
        self.0
    }

    /// Create an Energy struct from a value measuring joules.
    pub fn from_joules(joules: impl Into<f64>) -> Energy {
        Self::from_micro_joules((joules.into() * 1_000_000.0) as u32)
    }

    /// Return this Energy's value in joules.
    pub fn as_joules(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }
}

impl Raw for Energy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Energy::from_micro_joules)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_micro_joules().to_string())
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}J", self.as_joules())
    }
}

impl Add for Energy {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Energy(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Energy {
    type Output = Self;

    fn mul(self, other: T) -> Energy {
        Energy(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Energy {
    type Output = Self;

    fn div(self, other: T) -> Energy {
        Energy(self.0 / other.into())
    }
}
