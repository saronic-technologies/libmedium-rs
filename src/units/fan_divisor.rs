use crate::units::{Error as UnitError, Raw, Result as UnitResult};
use std::borrow::Cow;

/// Struct representing a fan divisor. Fan divisors can only be powers of two.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct FanDivisor(u32);

impl FanDivisor {
    /// Returns a FanDivisor created from a given value. If the value given is not a power of two
    /// an error is returned instead.
    pub fn try_from_value(value: impl Into<u32>) -> UnitResult<FanDivisor> {
        let value = value.into();

        if !value.is_power_of_two() {
            return Err(UnitError::invalid_value(value));
        }

        Ok(FanDivisor(value))
    }

    /// Returns the value stored in this `FanDivisor`.
    pub fn as_value(self) -> u32 {
        self.0
    }
}

impl Raw for FanDivisor {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(FanDivisor)
            .map_err(|_| UnitError::raw_conversion(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_bounds() {
        assert!(FanDivisor::try_from_value(0u32).is_err());
        assert!(FanDivisor::try_from_value(1u32).is_ok());
        assert!(FanDivisor::try_from_value(2u32).is_ok());
        assert!(FanDivisor::try_from_value(3u32).is_err());
    }
}
