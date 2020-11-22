use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::electric_current::milliampere as MilliAmps;

/// Type alias for `uom::si::electric_current::ElectricCurrent<uom::si::SI<f64>, f64>`.
pub type Current = uom::si::electric_current::ElectricCurrent<uom::si::SI<f64>, f64>;

impl Raw for Current {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Current::new::<MilliAmps>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<MilliAmps>().to_string())
    }
}
