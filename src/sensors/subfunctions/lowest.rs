use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all sensors that can have a lowest subfunction.
pub trait Lowest: Sensor {
    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}
