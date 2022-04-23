use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents an electrical current.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Current(i32);

impl Current {
    /// Creates a `Current` struct from a value measuring milliamperes.
    pub fn from_milli_amperes(millis: impl Into<i32>) -> Current {
        Current(millis.into())
    }

    /// Returns the struct's value in milliamperes.
    pub fn as_milli_amperes(self) -> i32 {
        self.0
    }

    /// Tries to create a `Current` struct from a value measuring amperes.
    pub fn try_from_amperes(amperes: impl Into<f64>) -> UnitResult<Current> {
        let amperes = amperes.into();

        if !amperes.is_finite()
            || amperes > f64::from(i32::MAX / 1_000)
            || amperes < f64::from(i32::MIN / 1_000)
        {
            return Err(UnitError::invalid_value(amperes));
        }

        Ok(Self::from_milli_amperes((amperes * 1_000.0) as i32))
    }

    /// Return this Current's value in amperes.
    pub fn as_amperes(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Current {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Current::from_milli_amperes)
            .map_err(UnitError::parsing)
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}A", self.as_amperes())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Current::try_from_amperes(f64::INFINITY).is_err());
        assert!(Current::try_from_amperes(f64::NEG_INFINITY).is_err());
        assert!(Current::try_from_amperes(f64::NAN).is_err());
        assert!(Current::try_from_amperes(-100.0).is_ok());
        assert!(Current::try_from_amperes(0.0).is_ok());
        assert!(Current::try_from_amperes(50.0).is_ok());
        assert!(Current::try_from_amperes(i32::MAX / 1_000).is_ok());
        assert!(Current::try_from_amperes(i32::MAX / 1_000 + 1).is_err());
        assert!(Current::try_from_amperes(i32::MIN / 1_000).is_ok());
        assert!(Current::try_from_amperes(i32::MIN / 1_000 - 1).is_err());
    }
}
