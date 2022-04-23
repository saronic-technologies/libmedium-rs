use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents an electrical voltage.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Voltage(i32);

impl Voltage {
    /// Creates a `Voltage` struct from a value measuring millivolts.
    pub fn from_milli_volts(millis: impl Into<i32>) -> Voltage {
        Voltage(millis.into())
    }

    /// Returns the struct's value in millivolts.
    pub fn as_milli_volts(self) -> i32 {
        self.0
    }

    /// Tries to create a `Voltage` struct from a value measuring volts.
    /// Returns an error if the given value is out of bounds.
    pub fn try_from_volts(volts: impl Into<f64>) -> UnitResult<Voltage> {
        let volts = volts.into();

        if !volts.is_finite()
            || volts > f64::from(i32::MAX / 1_000)
            || volts < f64::from(i32::MIN / 1_000)
        {
            return Err(UnitError::invalid_value(volts));
        }

        Ok(Self::from_milli_volts((volts * 1_000.0) as i32))
    }

    /// Return the struct's value in volts.
    pub fn as_volts(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Voltage {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Voltage::from_milli_volts)
            .map_err(UnitError::parsing)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Voltage::try_from_volts(f64::INFINITY).is_err());
        assert!(Voltage::try_from_volts(f64::NEG_INFINITY).is_err());
        assert!(Voltage::try_from_volts(f64::NAN).is_err());
        assert!(Voltage::try_from_volts(-100.0).is_ok());
        assert!(Voltage::try_from_volts(0.0).is_ok());
        assert!(Voltage::try_from_volts(50.0).is_ok());
        assert!(Voltage::try_from_volts(i32::MAX / 1_000).is_ok());
        assert!(Voltage::try_from_volts(i32::MAX / 1_000 + 1).is_err());
        assert!(Voltage::try_from_volts(i32::MIN / 1_000).is_ok());
        assert!(Voltage::try_from_volts(i32::MIN / 1_000 - 1).is_err());
    }
}
