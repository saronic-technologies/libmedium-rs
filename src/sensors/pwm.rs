//! Module containing the pwm sensors and their related functionality.

use super::*;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::{Frequency, Pwm, PwmEnable, PwmMode, Raw};

use std::path::Path;

/// Helper trait that sums up all functionality of a read-only pwm sensor.
pub trait PwmSensor: Sensor<Value = Pwm> + std::fmt::Debug {
    /// Reads the pwm subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_pwm(&self) -> Result<Pwm> {
        let raw = self.read_raw(SensorSubFunctionType::Pwm)?;
        Pwm::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the enable subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_enable(&self) -> Result<PwmEnable> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        PwmEnable::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the mode subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_mode(&self) -> Result<PwmMode> {
        let raw = self.read_raw(SensorSubFunctionType::Mode)?;
        PwmMode::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the freq subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_frequency(&self) -> Result<Frequency> {
        let raw = self.read_raw(SensorSubFunctionType::Freq)?;
        Frequency::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only pwm sensor.
#[derive(Debug, Clone)]
pub(crate) struct PwmSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for PwmSensorStruct {
    type Value = Pwm;

    fn base(&self) -> &'static str {
        "pwm"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for PwmSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(pwm, SensorSubFunctionType::Pwm)
    }
}

impl PwmSensor for PwmSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for PwmSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write pwm sensor.
pub trait WriteablePwmSensor: PwmSensor + WriteableSensor {
    /// Converts pwm and writes it to this pwm's pwm subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_pwm(&self, pwm: Pwm) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Pwm, &pwm.to_raw())
    }

    /// Converts enable and writes it to this pwm's enable subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_enable(&self, enable: PwmEnable) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }

    /// Converts mode and writes it to this pwm's mode subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_mode(&self, mode: PwmMode) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Mode, &mode.to_raw())
    }

    /// Converts freq and writes it to this pwm's freq subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_frequency(&self, freq: Frequency) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Freq, &freq.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteablePwmSensor for PwmSensorStruct {}
