use crate::units::{Raw, RawError, RawSensorResult};
use std::cmp::Ordering;
use std::fmt;
use std::borrow::Cow;

/// Struct that represents the accuracy of a power sensor.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Accuracy(u8);

impl Accuracy {
    /// Create a Accuracy struct from a value measuring percent.
    pub fn from_percent(percent: u8) -> Self {
        Self(percent)
    }

    /// Returns this struct's value as percent.
    pub fn as_percent(self) -> u8 {
        self.0
    }
}

impl Raw for Accuracy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u8>()
            .map(Accuracy::from_percent)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_percent().to_string())
    }
}

impl fmt::Display for Accuracy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

impl Eq for Accuracy {}

impl Ord for Accuracy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
