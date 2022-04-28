use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all sensors that can have an average subfunction.
pub trait Average: Sensor {
    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_average(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Average)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}
