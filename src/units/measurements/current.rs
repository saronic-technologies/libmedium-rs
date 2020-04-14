use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents an electrical current.
pub use measurements::Current;

impl Raw for Current {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Current::from_milliamperes)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        (self.as_milliamperes().round() as i32).to_string()
    }
}
