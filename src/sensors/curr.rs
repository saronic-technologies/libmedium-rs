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

//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::{Parseable, ParsingResult};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents an electrical current.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Current(i32);

impl Current {
    /// Create a Current struct from a value measuring milliamperes.
    pub fn from_milli_amperes(millis: i32) -> Current {
        Current(millis)
    }

    /// Return this Current's value in milliamperes.
    pub fn as_milli_amperes(self) -> i32 {
        self.0
    }

    /// Create a Current struct from a value measuring amperes.
    pub fn from_amperes(joules: impl Into<f64>) -> Current {
        Self::from_milli_amperes((joules.into() * 1_000.0) as i32)
    }

    /// Return this Current's value in amperes.
    pub fn as_amperes(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Current {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Current::from_milli_amperes)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Current {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}A", self.as_amperes())
    }
}

impl Eq for Current {}

impl Ord for Current {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Current {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Current(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Current {
    type Output = Self;

    fn mul(self, other: T) -> Current {
        Current(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Current {
    type Output = Self;

    fn div(self, other: T) -> Current {
        Current(self.0 / other.into())
    }
}

/// Trait implemented by all current sensors.
pub trait CurrSensor: SensorBase {}

impl<S: CurrSensor> Sensor<Current> for S {}
impl<S: CurrSensor> Min<Current> for S {}
impl<S: CurrSensor> Max<Current> for S {}
impl<S: CurrSensor> LowCrit<Current> for S {}
impl<S: CurrSensor> Crit<Current> for S {}
impl<S: CurrSensor> Average<Current> for S {}
impl<S: CurrSensor> Lowest<Current> for S {}
impl<S: CurrSensor> Highest<Current> for S {}

/// Struct that represents a read only current sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyCurr {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyCurr {
    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyCurr {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&curr) {
            Ok(curr)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "current sensor",
                index,
            })
        }
    }
}

impl CurrSensor for ReadOnlyCurr {}

#[cfg(feature = "writable")]
impl From<ReadWriteCurr> for ReadOnlyCurr {
    fn from(write_curr: ReadWriteCurr) -> ReadOnlyCurr {
        ReadOnlyCurr {
            hwmon_path: write_curr.hwmon_path,
            index: write_curr.index,
        }
    }
}

/// Struct that represents a read/write current sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteCurr {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteCurr {
    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteCurr {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&curr) {
            Ok(curr)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "current sensor",
                index,
            })
        }
    }
}

#[cfg(feature = "writable")]
impl CurrSensor for ReadWriteCurr {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteCurr {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyCurr> for ReadWriteCurr {
    type Error = SensorError;

    fn try_from(value: ReadOnlyCurr) -> Result<Self, Self::Error> {
        let read_write = ReadWriteCurr {
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
