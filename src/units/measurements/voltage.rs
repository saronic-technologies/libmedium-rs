use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents an electrical voltage.
pub use measurements::Voltage;

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Voltage::from_millivolts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        (self.as_millivolts().round() as i32).to_string()
    }
}
