use crate::units::{Raw, RawError, RawSensorResult};

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents an electrical current.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Current(i32);

impl Current {
    /// Create a Current struct from a value measuring milliamperes.
    pub fn from_milli_amperes(millis: i32) -> Current {
        Current(millis)
    }

    /// Return this Current's value in milliamperes.
    pub fn as_milli_amperes(self) -> i32 {
        self.0
    }

    /// Create a Current struct from a value measuring amperes.
    pub fn from_amperes(joules: impl Into<f64>) -> Current {
        Self::from_milli_amperes((joules.into() * 1_000.0) as i32)
    }

    /// Return this Current's value in amperes.
    pub fn as_amperes(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Current {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Current::from_milli_amperes)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}A", self.as_amperes())
    }
}

impl Eq for Current {}

impl Ord for Current {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Current {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Current(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Current {
    type Output = Self;

    fn mul(self, other: T) -> Current {
        Current(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Current {
    type Output = Self;

    fn div(self, other: T) -> Current {
        Current(self.0 / other.into())
    }
}
