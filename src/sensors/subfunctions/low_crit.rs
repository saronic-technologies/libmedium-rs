use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors that can have a lcrit subfunction.
pub trait LowCrit: Sensor {
    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lcrit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a lcrit subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableLowCrit: WriteableSensor {
    /// Writes this sensor's lcrit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_lcrit(&self, lcrit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + LowCrit> WriteableLowCrit for S {}
