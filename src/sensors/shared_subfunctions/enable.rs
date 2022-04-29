use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to read the enable subfunction.
pub trait Enable: Sensor {
    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors except for pwm.
#[cfg(feature = "writeable")]
pub trait WriteableEnable: WriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Enable> WriteableEnable for S {}
