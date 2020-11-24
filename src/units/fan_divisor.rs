use crate::units::{Error as RawError, Raw, Result as RawSensorResult};
use std::borrow::Cow;

/// Struct representing a fan divisor. Fan divisors can only be powers of two.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct FanDivisor(u32);

impl FanDivisor {
    /// Returns a FanDivisor created from a given value. If the value given is not a power of two
    /// the next higher power of two is chosen instead.
    pub fn from_value(value: u32) -> FanDivisor {
        FanDivisor(value.next_power_of_two())
    }

    /// Returns the value stored in this `FanDivisor`.
    pub fn as_value(self) -> u32 {
        self.0
    }
}

impl Raw for FanDivisor {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(FanDivisor)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }
}
