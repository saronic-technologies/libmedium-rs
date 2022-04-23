use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents a ratio. It is used for humidity and accuracy measurements.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Ratio(u32);

impl Ratio {
    /// Creates a `Ratio` struct from a value measuring millipercent.
    pub fn from_milli_percent(millis: u32) -> Self {
        Self(millis)
    }

    /// Returns the struct's value as millipercent.
    pub fn as_milli_percent(self) -> u32 {
        self.0
    }

    /// Returns the struct's value as percent.
    pub fn as_percent(self) -> f64 {
        f64::from(self.0) / 1000.0
    }
}

impl Raw for Ratio {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Ratio::from_milli_percent)
            .map_err(UnitError::parsing)
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_milli_percent().to_string())
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

impl Add for Ratio {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Ratio(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Ratio {
    type Output = Self;

    fn mul(self, other: T) -> Ratio {
        Ratio(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Ratio {
    type Output = Self;

    fn div(self, other: T) -> Ratio {
        Ratio(self.0 / other.into())
    }
}
