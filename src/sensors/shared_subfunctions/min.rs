use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors that can have a min subfunction.
pub trait Min: Sensor {
    /// Reads this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_min(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Min)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a min subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableMin: WriteableSensor {
    /// Writes this sensor's min value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_min(&self, min: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Min, &min.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Min> WriteableMin for S {}
