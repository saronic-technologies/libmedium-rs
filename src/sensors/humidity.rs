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
use crate::Parseable;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents humidity.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Humidity(u32);

impl Humidity {
    /// Create a Humidity struct from a value measuring millipercent.
    pub fn from_milli_percent(millis: u32) -> Self {
        Self(millis)
    }

    /// Returns this struct's value as millipercent.
    pub fn as_milli_percent(self) -> u32 {
        self.0
    }

    /// Returns this struct's value as percent.
    pub fn as_percent(self) -> f64 {
        f64::from(self.0) / 1000.0
    }
}

impl Raw for Humidity {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Humidity::from_milli_percent)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_milli_percent().to_string()
    }
}

impl fmt::Display for Humidity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

impl Eq for Humidity {}

impl Ord for Humidity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Humidity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Humidity(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Humidity {
    type Output = Self;

    fn mul(self, other: T) -> Humidity {
        Humidity(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Humidity {
    type Output = Self;

    fn div(self, other: T) -> Humidity {
        Humidity(self.0 / other.into())
    }
}

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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&humidity)?;

        Ok(humidity)
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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&humidity)?;

        Ok(humidity)
    }
}

#[cfg(feature = "writable")]
impl HumiditySensor for ReadWriteHumidity {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteHumidity {}
