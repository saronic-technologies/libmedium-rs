use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors that can have a max subfunction.
pub trait Max: Sensor {
    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_max(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Max)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a max subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableMax: WriteableSensor {
    /// Writes this sensor's max value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_max(&self, max: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Max> WriteableMax for S {}
