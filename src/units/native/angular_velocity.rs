use crate::units::{Raw, RawError, RawSensorResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents a frequency.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct AngularVelocity(u32);

impl AngularVelocity {
    /// Create a AngularVelocity struct from a value measuring times per minute.
    pub fn from_rpm(rpm: u32) -> Self {
        AngularVelocity(rpm)
    }

    /// Return this AngularVelocity's value in times per minute.
    pub fn as_rpm(self) -> u32 {
        self.0
    }
}

impl Raw for AngularVelocity {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(AngularVelocity::from_rpm)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_rpm().to_string())
    }
}

impl fmt::Display for AngularVelocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}rpm", self.as_rpm())
    }
}

impl Add for AngularVelocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        AngularVelocity(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for AngularVelocity {
    type Output = Self;

    fn mul(self, other: T) -> AngularVelocity {
        AngularVelocity(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for AngularVelocity {
    type Output = Self;

    fn div(self, other: T) -> AngularVelocity {
        AngularVelocity(self.0 / other.into())
    }
}
