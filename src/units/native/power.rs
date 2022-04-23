use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents electrical power.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Power(u32);

impl Power {
    /// Tries to create a `Power` struct from a value measuring watts.
    /// Returns an error if the given value is out of bounds.
    pub fn try_from_watts(watts: impl Into<f64>) -> UnitResult<Self> {
        let watts = watts.into();

        if !watts.is_finite() || watts < 0.0 || watts > f64::from(u32::MAX / 1_000_000) {
            return Err(UnitError::invalid_value(watts));
        }

        Ok(Self((watts * 1_000_000.0) as u32))
    }

    /// Creates a `Power` struct from a value measuring microwatts.
    pub fn from_microwatts(microwatts: impl Into<u32>) -> Self {
        Self(microwatts.into())
    }

    /// Returns the struct's value as watts.
    pub fn as_watts(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }

    /// Returns the struct's value as microwatts.
    pub fn as_microwatts(self) -> u32 {
        self.0
    }
}

impl Raw for Power {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Power::from_microwatts)
            .map_err(UnitError::parsing)
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_microwatts().to_string())
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}W", self.as_watts())
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Power(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Power {
    type Output = Self;

    fn mul(self, other: T) -> Power {
        Power(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Power {
    type Output = Self;

    fn div(self, other: T) -> Power {
        Power(self.0 / other.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Power::try_from_watts(f64::INFINITY).is_err());
        assert!(Power::try_from_watts(f64::NEG_INFINITY).is_err());
        assert!(Power::try_from_watts(f64::NAN).is_err());
        assert!(Power::try_from_watts(-100.0).is_err());
        assert!(Power::try_from_watts(0.0).is_ok());
        assert!(Power::try_from_watts(50.0).is_ok());
        assert!(Power::try_from_watts(u32::MAX / 1_000_000).is_ok());
        assert!(Power::try_from_watts(u32::MAX / 1_000_000 + 1).is_err());
    }
}
