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

//! Module containing the power sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::Parseable;

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul};

/// Struct that represents electrical power.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Power(u32);

impl Power {
    /// Create a Power struct from a value measuring watts.
    pub fn from_watts(degrees: impl Into<f64>) -> Self {
        Self((degrees.into() * 1_000_000.0) as u32)
    }

    /// Create a Power struct from a value measuring microwatts.
    pub fn from_microwatts(microwatts: impl Into<u32>) -> Self {
        Self(microwatts.into())
    }

    /// Returns this struct's value as watts.
    pub fn as_watts(self) -> f64 {
        f64::from(self.0) / 1_000_000.0
    }

    /// Returns this struct's value as microwatts.
    pub fn as_microwatts(self) -> u32 {
        self.0
    }
}

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Power::from_microwatts)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_microwatts().to_string()
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}W", self.as_watts())
    }
}

impl Eq for Power {}

impl Ord for Power {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Power(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Power {
    type Output = Self;

    fn mul(self, other: T) -> Power {
        Power(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Power {
    type Output = Self;

    fn div(self, other: T) -> Power {
        Power(self.0 / other.into())
    }
}

/// Struct that represents the accuracy of a power sensor.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Accuracy(u8);

impl Accuracy {
    /// Create a Power struct from a value measuring watts.
    pub fn from_percent(percent: u8) -> Self {
        Self(percent)
    }

    /// Returns this struct's value as watts.
    pub fn as_percent(self) -> u8 {
        self.0
    }
}

impl Raw for Accuracy {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u8>()
            .map(Accuracy::from_percent)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_percent().to_string()
    }
}

impl fmt::Display for Accuracy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

impl Eq for Accuracy {}

impl Ord for Accuracy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

/// Trait implemented by all power sensors.
pub trait PowerSensor: SensorBase {
    /// Reads the accuracy subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_accuracy(&self) -> SensorResult<Accuracy> {
        let raw = self.read_raw(SensorSubFunctionType::Accuracy)?;
        Accuracy::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::Cap)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the cap_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_max(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMax)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the cap_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_min(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMin)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_hyst(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapHyst)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval(&self) -> SensorResult<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageInterval)?;
        Duration::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_interval_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_max(&self) -> SensorResult<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMax)?;
        Duration::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_interval_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_min(&self) -> SensorResult<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMin)?;
        Duration::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_highest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_highest(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageHighest)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_lowest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_lowest(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageLowest)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_max(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMax)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the average_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_min(&self) -> SensorResult<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMin)?;
        Power::from_raw(&raw).map_err(SensorError::from)
    }

    /// Converts cap and writes it to the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_cap(&self, cap: Power) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Cap, &cap.to_raw())
    }

    /// Converts cap_hyst and writes it to the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_cap_hyst(&self, cap_hyst: Power) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::CapHyst, &cap_hyst.to_raw())
    }

    /// Converts interval and writes it to the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_average_interval(&self, interval: Duration) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::AverageInterval, &interval.to_raw())
    }
}

impl<S: PowerSensor> Sensor<Power> for S {}
impl<S: PowerSensor> Max<Power> for S {}
impl<S: PowerSensor> Crit<Power> for S {}
impl<S: PowerSensor> Average<Power> for S {}
impl<S: PowerSensor> Highest<Power> for S {}
impl<S: PowerSensor> Lowest<Power> for S {}

/// Struct that represents a read only power sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyPower {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyPower {
    fn base(&self) -> &'static str {
        "power"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyPower {
    type Parent = ReadOnlyHwmon;
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&power)?;

        Ok(power)
    }
}

impl PowerSensor for ReadOnlyPower {}

#[cfg(feature = "writable")]
impl From<ReadWritePower> for ReadOnlyPower {
    fn from(write_power: ReadWritePower) -> ReadOnlyPower {
        ReadOnlyPower {
            hwmon_path: write_power.hwmon_path,
            index: write_power.index,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&ReadWritePower> for ReadOnlyPower {
    fn from(write_power: &ReadWritePower) -> ReadOnlyPower {
        ReadOnlyPower {
            hwmon_path: write_power.hwmon_path().to_path_buf(),
            index: write_power.index(),
        }
    }
}

/// Struct that represents a read/write power sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWritePower {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWritePower {
    fn base(&self) -> &'static str {
        "power"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWritePower {
    type Parent = ReadWriteHwmon;
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&power)?;

        Ok(power)
    }
}

#[cfg(feature = "writable")]
impl PowerSensor for ReadWritePower {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWritePower {}
