use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents used energy.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Energy(u32);

impl Energy {
    /// Creates an `Energy` struct from a value measuring microjoules.
    pub fn from_micro_joules(micros: impl Into<u32>) -> Energy {
        Energy(micros.into())
    }

    /// Returns the struct's value in microjoules.
    pub fn as_micro_joules(self) -> u32 {
        self.0
    }

    /// Tries to create an `Energy` struct from a value measuring joules.
    /// Returns an error if the given value is out of bounds.
    pub fn try_from_joules(joules: impl Into<f64>) -> UnitResult<Energy> {
        let joules = joules.into();

        if !joules.is_finite() || joules < 0.0 || joules > f64::from(u32::MAX / 1_000_000) {
            return Err(UnitError::invalid_value(joules));
        }

        Ok(Self::from_micro_joules((joules * 1_000_000.0) as u32))
    }

    /// Return this Energy's value in joules.
    pub fn as_joules(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }
}

impl Raw for Energy {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Energy::from_micro_joules)
            .map_err(|_| UnitError::raw_conversion(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_micro_joules().to_string())
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}J", self.as_joules())
    }
}

impl Add for Energy {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Energy(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Energy {
    type Output = Self;

    fn mul(self, other: T) -> Energy {
        Energy(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Energy {
    type Output = Self;

    fn div(self, other: T) -> Energy {
        Energy(self.0 / other.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Energy::try_from_joules(f64::INFINITY).is_err());
        assert!(Energy::try_from_joules(f64::NEG_INFINITY).is_err());
        assert!(Energy::try_from_joules(f64::NAN).is_err());
        assert!(Energy::try_from_joules(-100.0).is_err());
        assert!(Energy::try_from_joules(0.0).is_ok());
        assert!(Energy::try_from_joules(50.0).is_ok());
        assert!(Energy::try_from_joules(u32::MAX / 1_000_000).is_ok());
        assert!(Energy::try_from_joules(u32::MAX / 1_000_000 + 1).is_err());
    }
}
