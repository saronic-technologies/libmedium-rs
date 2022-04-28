use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType, WriteableSensor},
    units::Raw,
};

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to read the beep subfunction.
pub trait Beep: Sensor {
    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors except for pwm.
#[cfg(feature = "writeable")]
pub trait WriteableBeep: WriteableSensor {
    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Beep> WriteableBeep for S {}
