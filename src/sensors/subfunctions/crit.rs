use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors that can have a crit subfunction.
pub trait Crit: Sensor {
    /// Reads this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_crit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Crit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a crit subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableCrit: WriteableSensor {
    /// Writes this sensor's crit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_crit(&self, crit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Crit> WriteableCrit for S {}
