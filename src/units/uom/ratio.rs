use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::ratio::percent as Percent;

/// Type alias for `uom::si::ratio::Ratio<uom::si::SI<f64>, f64>`.
pub type Ratio = uom::si::ratio::Ratio<uom::si::SI<f64>, f64>;

impl Raw for Ratio {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Ratio::new::<Percent>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<Percent>().to_string())
    }
}
