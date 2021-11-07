use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents a temperature.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Temperature(i32);

impl Temperature {
    /// Creates a `Temperature` struct from a value measuring millidegrees celsius.
    pub fn from_millidegrees_celsius(millidegrees: impl Into<i32>) -> Self {
        Self(millidegrees.into())
    }

    /// Tries to create a `Temperature` struct from a value measuring degrees celsius.
    /// Returns an error if the given value is out of bounds.
    pub fn try_from_degrees_celsius(degrees: impl Into<f64>) -> UnitResult<Self> {
        let degrees = degrees.into();

        if !degrees.is_finite()
            || degrees > f64::from(i32::MAX / 1_000)
            || degrees < f64::from(i32::MIN / 1_000)
        {
            return Err(UnitError::invalid_value(degrees));
        }

        Ok(Self((degrees * 1_000.0) as i32))
    }

    /// Create a Temperature struct from a value measuring degrees fahrenheit.
    pub fn try_from_degrees_fahrenheit(degrees: impl Into<f64>) -> UnitResult<Self> {
        Self::try_from_degrees_celsius((degrees.into() - 32.0) / 1.8)
    }

    /// Returns the struct's value as degrees celsius.
    pub fn as_degrees_celsius(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }

    /// Returns the struct's value as millidegrees celsius.amperes
    pub fn as_millidegrees_celsius(self) -> i32 {
        self.0
    }

    /// Returns the struct's value as degrees fahrenheit.
    pub fn as_degrees_fahrenheit(self) -> f64 {
        self.as_degrees_celsius() * 1.8 + 32.0
    }
}

impl Raw for Temperature {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Temperature::from_millidegrees_celsius)
            .map_err(|_| UnitError::raw_conversion(raw))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(Temperature::try_from_degrees_celsius(f64::INFINITY).is_err());
        assert!(Temperature::try_from_degrees_celsius(f64::NEG_INFINITY).is_err());
        assert!(Temperature::try_from_degrees_celsius(f64::NAN).is_err());
        assert!(Temperature::try_from_degrees_celsius(-100.0).is_ok());
        assert!(Temperature::try_from_degrees_celsius(0.0).is_ok());
        assert!(Temperature::try_from_degrees_celsius(50.0).is_ok());
        assert!(Temperature::try_from_degrees_celsius(i32::MAX / 1_000).is_ok());
        assert!(Temperature::try_from_degrees_celsius(i32::MAX / 1_000 + 1).is_err());
        assert!(Temperature::try_from_degrees_celsius(i32::MIN / 1_000).is_ok());
        assert!(Temperature::try_from_degrees_celsius(i32::MIN / 1_000 - 1).is_err());
    }
}
