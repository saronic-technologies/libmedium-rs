use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents electrical power.
pub use measurements::Power;

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Power::from_microwatts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_microwatts().round().to_string()
    }
}
