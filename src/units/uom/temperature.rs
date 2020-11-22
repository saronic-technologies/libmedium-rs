use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::thermodynamic_temperature::millikelvin as MilliKelvin;

/// Type alias for `uom::si::thermodynamic_temperature::ThermodynamicTemperature<uom::si::SI<f64>, f64>`.
pub type Temperature =
    uom::si::thermodynamic_temperature::ThermodynamicTemperature<uom::si::SI<f64>, f64>;

impl Raw for Temperature {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(|celsius| celsius + 273150.0)
            .map(Temperature::new::<MilliKelvin>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned((self.get::<MilliKelvin>() - 273150.0).to_string())
    }
}
