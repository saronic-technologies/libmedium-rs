use crate::units::{Raw, RawError, RawSensorResult};

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents a frequency.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Frequency(u32);

impl Frequency {
    /// Create a Frequency struct from a value measuring times per minute.
    pub fn from_times_per_minute(rpm: u32) -> Frequency {
        Frequency(rpm)
    }

    /// Return this Frequency's value in times per minute.
    pub fn as_times_per_minute(self) -> u32 {
        self.0
    }
}

impl Raw for Frequency {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Frequency::from_times_per_minute)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_times_per_minute().to_string()
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}rpm", self.as_times_per_minute())
    }
}

impl Eq for Frequency {}

impl Ord for Frequency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
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
