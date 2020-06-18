use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents an electrical voltage.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Voltage(i32);

impl Voltage {
    /// Create a Voltage struct from a value measuring millivolts.
    pub fn from_milli_volts(millis: i32) -> Voltage {
        Voltage(millis)
    }

    /// Return this Voltage's value in millivolts.
    pub fn as_milli_volts(self) -> i32 {
        self.0
    }

    /// Create a Voltage struct from a value measuring volts.
    pub fn from_volts(volts: impl Into<f64>) -> Voltage {
        Self::from_milli_volts((volts.into() * 1_000.0) as i32)
    }

    /// Return this Voltage's value in volts.
    pub fn as_volts(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Voltage::from_milli_volts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}V", self.as_volts())
    }
}

impl Add for Voltage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Voltage(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Voltage {
    type Output = Self;

    fn mul(self, other: T) -> Voltage {
        Voltage(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Voltage {
    type Output = Self;

    fn div(self, other: T) -> Voltage {
        Voltage(self.0 / other.into())
    }
}
