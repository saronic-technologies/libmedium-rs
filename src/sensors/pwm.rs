//! Module containing the pwm sensors and their related functionality.

use super::*;
use crate::units::{Frequency, Pwm, PwmEnable, PwmMode, Raw};
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
use std::convert::TryFrom;
use std::path::Path;

/// Trait implemented by all pwm sensors.
pub trait PwmSensor: SensorBase {
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

    /// Converts pwm and writes it to this pwm's pwm subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_pwm(&self, pwm: Pwm) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Pwm, &pwm.to_raw())
    }

    /// Converts enable and writes it to this pwm's enable subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_enable(&self, enable: PwmEnable) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }

    /// Converts mode and writes it to this pwm's mode subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_mode(&self, mode: PwmMode) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Mode, &mode.to_raw())
    }

    /// Converts freq and writes it to this pwm's freq subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_frequency(&self, freq: Frequency) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Freq, &freq.to_raw())
    }
}

/// Struct that represents a read only pwm sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyPwm {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadOnlyPwm {
    /// Try converting this sensor into a read-write version of itself.
    pub fn try_into_read_write(self) -> Result<ReadWritePwm> {
        let read_write = ReadWritePwm {
            hwmon_path: self.hwmon_path,
            index: self.index,
        };

        if read_write.supported_write_sub_functions().is_empty() {
            return Err(Error::InsufficientRights {
                path: read_write.hwmon_path.join(format!(
                    "{}{}",
                    read_write.base(),
                    read_write.index(),
                )),
            });
        }

        Ok(read_write)
    }
}

impl SensorBase for ReadOnlyPwm {
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

impl Parseable for ReadOnlyPwm {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(pwm)
    }
}

impl PwmSensor for ReadOnlyPwm {}

#[cfg(feature = "writable")]
impl From<ReadWritePwm> for ReadOnlyPwm {
    fn from(write_pwm: ReadWritePwm) -> ReadOnlyPwm {
        write_pwm.into_read_only()
    }
}

/// Struct that represents a read/write pwm sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWritePwm {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadWritePwm {
    /// Converts this sensor into a read-only version of itself.
    pub fn into_read_only(self) -> ReadOnlyPwm {
        ReadOnlyPwm {
            hwmon_path: self.hwmon_path,
            index: self.index,
        }
    }
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWritePwm {
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

#[cfg(feature = "writable")]
impl Parseable for ReadWritePwm {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(pwm)
    }
}

#[cfg(feature = "writable")]
impl PwmSensor for ReadWritePwm {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWritePwm {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyPwm> for ReadWritePwm {
    type Error = Error;

    fn try_from(read_only: ReadOnlyPwm) -> std::result::Result<Self, Self::Error> {
        read_only.try_into_read_write()
    }
}
