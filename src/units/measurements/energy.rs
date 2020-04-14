use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents used energy.
pub use measurements::Energy;

impl Raw for Energy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(|raw| Energy::from_joules(raw / 1_000_000.0))
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        ((self.as_joules() * 1_000_000.0).round() as i32).to_string()
    }
}
