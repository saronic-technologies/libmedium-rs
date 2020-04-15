/*
 * Copyright (C) 2019  Malte Veerman <malte.veerman@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

//! Module containing the pwm sensors and their related functionality.

use super::*;
use crate::units::{Frequency, Pwm, PwmEnable, PwmMode, Raw};
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::Path;

/// Trait implemented by all pwm sensors.
pub trait PwmSensor: SensorBase {
    /// Reads the pwm subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_pwm(&self) -> SensorResult<Pwm> {
        let raw = self.read_raw(SensorSubFunctionType::Pwm)?;
        Pwm::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the enable subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_enable(&self) -> SensorResult<PwmEnable> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        PwmEnable::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the mode subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_mode(&self) -> SensorResult<PwmMode> {
        let raw = self.read_raw(SensorSubFunctionType::Mode)?;
        PwmMode::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the freq subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_frequency(&self) -> SensorResult<Frequency> {
        let raw = self.read_raw(SensorSubFunctionType::Freq)?;
        Frequency::from_raw(&raw).map_err(SensorError::from)
    }

    /// Converts pwm and writes it to this pwm's pwm subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_pwm(&self, pwm: Pwm) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Pwm, &pwm.to_raw())
    }

    /// Converts enable and writes it to this pwm's enable subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_enable(&self, enable: PwmEnable) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }

    /// Converts mode and writes it to this pwm's mode subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_mode(&self, mode: PwmMode) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Mode, &mode.to_raw())
    }

    /// Converts freq and writes it to this pwm's freq subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_frequency(&self, freq: Frequency) -> SensorResult<()>
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
        ReadOnlyPwm {
            hwmon_path: write_pwm.hwmon_path,
            index: write_pwm.index,
        }
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
    type Error = SensorError;

    fn try_from(value: ReadOnlyPwm) -> Result<Self, Self::Error> {
        let read_write = ReadWritePwm {
            hwmon_path: value.hwmon_path,
            index: value.index,
        };

        if read_write.supported_write_sub_functions().is_empty() {
            return Err(SensorError::InsufficientRights {
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
