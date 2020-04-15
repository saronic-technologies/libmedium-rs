use crate::units::{Raw, RawError, RawSensorResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents a temperature.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Temperature(i32);

impl Temperature {
    /// Create a Temperature struct from a value measuring degrees celsius.
    pub fn from_degrees_celsius(degrees: impl Into<f64>) -> Self {
        Self((degrees.into() * 1000.0) as i32)
    }

    /// Create a Temperature struct from a value measuring millidegrees celsius.
    pub fn from_millidegrees_celsius(millidegrees: impl Into<i32>) -> Self {
        Self(millidegrees.into())
    }

    /// Create a Temperature struct from a value measuring degrees fahrenheit.
    pub fn from_degrees_fahrenheit(degrees: impl Into<f64>) -> Self {
        Self::from_degrees_celsius((degrees.into() - 32.0) / 1.8)
    }

    /// Returns this struct's value as degrees celsius.
    pub fn as_degrees_celsius(self) -> f64 {
        f64::from(self.0) / 1000.0
    }

    /// Returns this struct's value as millidegrees celsius.
    pub fn as_millidegrees_celsius(self) -> i32 {
        self.0
    }

    /// Returns this struct's value as degrees fahrenheit.
    pub fn as_degrees_fahrenheit(self) -> f64 {
        self.as_degrees_celsius() * 1.8 + 32.0
    }
}

impl Raw for Temperature {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Temperature::from_millidegrees_celsius)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_millidegrees_celsius().to_string())
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.as_degrees_celsius())
    }
}

impl Add for Temperature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Temperature(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Temperature {
    type Output = Self;

    fn mul(self, other: T) -> Temperature {
        Temperature(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Temperature {
    type Output = Self;

    fn div(self, other: T) -> Temperature {
        Temperature(self.0 / other.into())
    }
}
