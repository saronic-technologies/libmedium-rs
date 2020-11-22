use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::electric_potential::millivolt as MilliVolt;

/// Type alias for `uom::si::electric_potential::ElectricPotential<uom::si::SI<f64>, f64>`.
pub type Voltage = uom::si::electric_potential::ElectricPotential<uom::si::SI<f64>, f64>;

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Voltage::new::<MilliVolt>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<MilliVolt>().to_string())
    }
}
