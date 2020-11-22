use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::frequency::hertz as Hertz;

/// Type alias for `uom::si::frequency::Frequency<uom::si::SI<f64>, f64>`.
pub type Frequency = uom::si::frequency::Frequency<uom::si::SI<f64>, f64>;

impl Raw for Frequency {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Frequency::new::<Hertz>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<Hertz>().to_string())
    }
}
