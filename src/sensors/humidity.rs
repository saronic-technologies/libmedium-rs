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

//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Humidity;
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all humidity sensors.
pub trait HumiditySensor: SensorBase {}

impl<S: HumiditySensor> Sensor<Humidity> for S {}

/// Struct that represents a read only humidity sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyHumidity {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyHumidity {
    fn base(&self) -> &'static str {
        "humidity"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyHumidity {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity)
    }
}

impl HumiditySensor for ReadOnlyHumidity {}

#[cfg(feature = "writable")]
impl From<ReadWriteHumidity> for ReadOnlyHumidity {
    fn from(write_humidity: ReadWriteHumidity) -> ReadOnlyHumidity {
        ReadOnlyHumidity {
            hwmon_path: write_humidity.hwmon_path,
            index: write_humidity.index,
        }
    }
}

/// Struct that represents a read/write humidity sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteHumidity {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteHumidity {
    fn base(&self) -> &'static str {
        "humidity"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteHumidity {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity)
    }
}

#[cfg(feature = "writable")]
impl HumiditySensor for ReadWriteHumidity {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteHumidity {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyHumidity> for ReadWriteHumidity {
    type Error = SensorError;

    fn try_from(value: ReadOnlyHumidity) -> Result<Self, Self::Error> {
        let read_write = ReadWriteHumidity {
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
