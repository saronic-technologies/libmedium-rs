use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::angular_velocity::revolution_per_minute as RPM;

/// Type alias for `uom::si::angular_velocity::AngularVelocity<uom::si::SI<f64>, f64>`.
pub type AngularVelocity = uom::si::angular_velocity::AngularVelocity<uom::si::SI<f64>, f64>;

impl Raw for AngularVelocity {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(AngularVelocity::new::<RPM>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.get::<RPM>().to_string())
    }
}
