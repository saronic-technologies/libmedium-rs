use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents an angular velocity.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct AngularVelocity(u32);

impl AngularVelocity {
    /// Creates an `AngularVelocity` struct from a value measuring revolutions per minute.
    pub fn from_rpm(rpm: impl Into<u32>) -> Self {
        AngularVelocity(rpm.into())
    }

    /// Returns the struct's value in revolutions per minute.
    pub fn as_rpm(self) -> u32 {
        self.0
    }
}

impl Raw for AngularVelocity {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(AngularVelocity::from_rpm)
            .map_err(|_| UnitError::raw_conversion(raw))
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
