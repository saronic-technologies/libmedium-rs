use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::energy::microjoule as MicroJoules;

/// Type alias for `uom::si::energy::Energy<uom::si::SI<f64>, f64>`.
pub type Energy = uom::si::energy::Energy<uom::si::SI<f64>, f64>;

impl Raw for Energy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Energy::new::<MicroJoules>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<MicroJoules>().to_string())
    }
}
