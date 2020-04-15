use crate::units::{Raw, RawError, RawSensorResult};

use ::measurements;

/// Struct that represents an electrical current.
pub use measurements::AngularVelocity;

use std::borrow::Cow;

impl Raw for AngularVelocity {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(AngularVelocity::from_rpm)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(self.as_rpm().round().to_string())
    }
}