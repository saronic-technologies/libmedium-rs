use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all sensors that can have a highest subfunction.
pub trait Highest: Sensor {
    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}
