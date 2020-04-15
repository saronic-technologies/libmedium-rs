use crate::units::{Raw, RawError, RawSensorResult};

use std::fmt;
use std::ops::{Add, Div, Mul};
use std::borrow::Cow;

/// Struct that represents a frequency.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash, Ord, Eq)]
pub struct Frequency(u32);

impl Frequency {
    /// Create a Frequency struct from a value measuring times per second.
    pub fn from_hertz(hz: u32) -> Self {
        Frequency(hz)
    }

    /// Return this Frequency's value in times per second.
    pub fn as_hertz(self) -> u32 {
        self.0
    }
}

impl Raw for Frequency {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Frequency::from_hertz)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_hertz().to_string())
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}hz", self.as_hertz())
    }
}

impl Add for Frequency {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Frequency(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Frequency {
    type Output = Self;

    fn mul(self, other: T) -> Frequency {
        Frequency(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Frequency {
    type Output = Self;

    fn div(self, other: T) -> Frequency {
        Frequency(self.0 / other.into())
    }
}
