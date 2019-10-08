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
use crate::Parseable;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents used energy.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Energy(u32);

impl Energy {
    /// Create an Energy struct from a value measuring microjoules.
    pub fn from_micro_joules(micros: u32) -> Energy {
        Energy(micros)
    }

    /// Return this Energy's value in microjoules.
    pub fn as_micro_joules(self) -> u32 {
        self.0
    }

    /// Create an Energy struct from a value measuring joules.
    pub fn from_joules(joules: impl Into<f64>) -> Energy {
        Self::from_micro_joules((joules.into() * 1_000_000.0) as u32)
    }

    /// Return this Energy's value in joules.
    pub fn as_joules(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }
}

impl Raw for Energy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Energy::from_micro_joules)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_micro_joules().to_string()
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}J", self.as_joules())
    }
}

impl Eq for Energy {}

impl Ord for Energy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Energy {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Energy(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Energy {
    type Output = Self;

    fn mul(self, other: T) -> Energy {
        Energy(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Energy {
    type Output = Self;

    fn div(self, other: T) -> Energy {
        Energy(self.0 / other.into())
    }
}

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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&energy)?;

        Ok(energy)
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

#[cfg(feature = "writable")]
impl From<&ReadWriteEnergy> for ReadOnlyEnergy {
    fn from(write_energy: &ReadWriteEnergy) -> ReadOnlyEnergy {
        ReadOnlyEnergy {
            hwmon_path: write_energy.hwmon_path().to_path_buf(),
            index: write_energy.index(),
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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&energy)?;

        Ok(energy)
    }
}

#[cfg(feature = "writable")]
impl EnergySensor for ReadWriteEnergy {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteEnergy {}
