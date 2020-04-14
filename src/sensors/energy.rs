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

//! Module containing the energy sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Energy;
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all energy sensors.
pub trait EnergySensor: SensorBase {}

impl<S: EnergySensor> Sensor<Energy> for S {}

/// Struct that represents a read only energy sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyEnergy {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyEnergy {
    fn base(&self) -> &'static str {
        "energy"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyEnergy {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&energy) {
            Ok(energy)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "energy sensor",
                index,
            })
        }
    }
}

impl EnergySensor for ReadOnlyEnergy {}

#[cfg(feature = "writable")]
impl From<ReadWriteEnergy> for ReadOnlyEnergy {
    fn from(write_energy: ReadWriteEnergy) -> ReadOnlyEnergy {
        ReadOnlyEnergy {
            hwmon_path: write_energy.hwmon_path,
            index: write_energy.index,
        }
    }
}

/// Struct that represents a read/write energy sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteEnergy {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteEnergy {
    fn base(&self) -> &'static str {
        "energy"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteEnergy {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&energy) {
            Ok(energy)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "energy sensor",
                index,
            })
        }
    }
}

#[cfg(feature = "writable")]
impl EnergySensor for ReadWriteEnergy {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteEnergy {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyEnergy> for ReadWriteEnergy {
    type Error = SensorError;

    fn try_from(value: ReadOnlyEnergy) -> Result<Self, Self::Error> {
        let read_write = ReadWriteEnergy {
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
