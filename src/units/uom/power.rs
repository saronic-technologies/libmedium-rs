use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::power::microwatt as MicroWatt;

/// Type alias for `uom::si::power::Power<uom::si::SI<f64>, f64>`.
pub type Power = uom::si::power::Power<uom::si::SI<f64>, f64>;

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Power::new::<MicroWatt>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<MicroWatt>().to_string())
    }
}
