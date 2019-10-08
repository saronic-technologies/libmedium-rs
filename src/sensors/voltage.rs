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

//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::Parseable;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents an electrical voltage.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Voltage(i32);

impl Voltage {
    /// Create a Voltage struct from a value measuring millivolts.
    pub fn from_milli_volts(millis: i32) -> Voltage {
        Voltage(millis)
    }

    /// Return this Voltage's value in millivolts.
    pub fn as_milli_volts(self) -> i32 {
        self.0
    }

    /// Create a Voltage struct from a value measuring volts.
    pub fn from_volts(volts: impl Into<f64>) -> Voltage {
        Self::from_milli_volts((volts.into() * 1_000.0) as i32)
    }

    /// Return this Voltage's value in volts.
    pub fn as_volts(self) -> f64 {
        f64::from(self.0) / 1_000.0
    }
}

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Voltage::from_milli_volts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}V", self.as_volts())
    }
}

impl Eq for Voltage {}

impl Ord for Voltage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Voltage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Voltage(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Voltage {
    type Output = Self;

    fn mul(self, other: T) -> Voltage {
        Voltage(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Voltage {
    type Output = Self;

    fn div(self, other: T) -> Voltage {
        Voltage(self.0 / other.into())
    }
}

/// Trait implemented by all voltage sensors.
pub trait VoltSensor: SensorBase {}

impl<S: VoltSensor> Sensor<Voltage> for S {}
impl<S: VoltSensor> Min<Voltage> for S {}
impl<S: VoltSensor> Max<Voltage> for S {}
impl<S: VoltSensor> LowCrit<Voltage> for S {}
impl<S: VoltSensor> Crit<Voltage> for S {}
impl<S: VoltSensor> Average<Voltage> for S {}
impl<S: VoltSensor> Lowest<Voltage> for S {}
impl<S: VoltSensor> Highest<Voltage> for S {}

/// Struct that represents a read only voltage sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyVolt {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyVolt {
    fn base(&self) -> &'static str {
        "in"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyVolt {
    type Parent = ReadOnlyHwmon;
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&volt)?;

        Ok(volt)
    }
}

impl VoltSensor for ReadOnlyVolt {}

#[cfg(feature = "writable")]
impl From<ReadWriteVolt> for ReadOnlyVolt {
    fn from(write_voltage: ReadWriteVolt) -> ReadOnlyVolt {
        ReadOnlyVolt {
            hwmon_path: write_voltage.hwmon_path,
            index: write_voltage.index,
        }
    }
}

/// Struct that represents a read/write voltage sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteVolt {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteVolt {
    fn base(&self) -> &'static str {
        "in"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteVolt {
    type Parent = ReadWriteHwmon;
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&volt)?;

        Ok(volt)
    }
}

#[cfg(feature = "writable")]
impl VoltSensor for ReadWriteVolt {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteVolt {}
