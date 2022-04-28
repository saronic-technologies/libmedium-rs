use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all sensors that can have a faulty subfunction.
pub trait Faulty: Sensor {
    /// Reads whether this sensor is faulty or not.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_faulty(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Fault)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}
