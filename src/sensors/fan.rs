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

//! Module containing the fan sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::{Parseable, ParsingResult};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents used Revs.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Revs(u32);

impl Revs {
    /// Create a Revs struct from a value measuring times per minute.
    pub fn from_times_per_minute(rpm: u32) -> Revs {
        Revs(rpm)
    }

    /// Return this Revs's value in times per minute.
    pub fn as_times_per_minute(self) -> u32 {
        self.0
    }
}

impl Raw for Revs {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Revs::from_times_per_minute)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_times_per_minute().to_string()
    }
}

impl fmt::Display for Revs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}rpm", self.as_times_per_minute())
    }
}

impl Eq for Revs {}

impl Ord for Revs {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Revs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Revs(self.0 + other.0)
    }
}

impl<T: Into<u32>> Mul<T> for Revs {
    type Output = Self;

    fn mul(self, other: T) -> Revs {
        Revs(self.0 * other.into())
    }
}

impl<T: Into<u32>> Div<T> for Revs {
    type Output = Self;

    fn div(self, other: T) -> Revs {
        Revs(self.0 / other.into())
    }
}

/// A struct representing a fan divisor. Fan divisors can only be powers of two.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct FanDivisor(u32);

impl FanDivisor {
    /// Returns a FanDivisor created from a given value. If the value given is not a power of two
    /// the next higher power of two is chosen instead.
    pub fn from_value(value: u32) -> FanDivisor {
        FanDivisor(value.next_power_of_two())
    }
}

impl Raw for FanDivisor {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(FanDivisor)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.0.to_string()
    }
}

/// Trait implemented by all fan sensors.
pub trait FanSensor: SensorBase {
    /// Reads the target_Revs subfunction of this fan sensor.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_target(&self) -> SensorResult<Revs> {
        let raw = self.read_raw(SensorSubFunctionType::Target)?;
        Revs::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the div subfunction of this fan sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_div(&self) -> SensorResult<FanDivisor> {
        let raw = self.read_raw(SensorSubFunctionType::Div)?;
        FanDivisor::from_raw(&raw).map_err(SensorError::from)
    }

    /// Converts target and writes it to this fan's target subfunction.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_target(&self, target: Revs) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Target, &target.to_raw())
    }

    /// Converts div and writes it to this fan's divisor subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_div(&self, div: FanDivisor) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Div, &div.to_raw())
    }
}

impl<S: FanSensor + Faulty> Sensor<Revs> for S {
    /// Reads the input subfunction of this fan sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_input(&self) -> SensorResult<Revs> {
        if self.read_faulty().unwrap_or(false) {
            return Err(SensorError::FaultySensor);
        }

        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Revs::from_raw(&raw).map_err(SensorError::from)
    }
}

impl<S: FanSensor> Min<Revs> for S {}
impl<S: FanSensor> Max<Revs> for S {}

/// Struct that represents a read only fan sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyFan {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyFan {
    fn base(&self) -> &'static str {
        "fan"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyFan {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let fan = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&fan) {
            Ok(fan)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "fan sensor",
                index,
            })
        }
    }
}

impl FanSensor for ReadOnlyFan {}
impl Faulty for ReadOnlyFan {}

#[cfg(feature = "writable")]
impl From<ReadWriteFan> for ReadOnlyFan {
    fn from(write_fan: ReadWriteFan) -> ReadOnlyFan {
        ReadOnlyFan {
            hwmon_path: write_fan.hwmon_path,
            index: write_fan.index,
        }
    }
}

/// Struct that represents a read/write fan sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteFan {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteFan {
    fn base(&self) -> &'static str {
        "fan"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteFan {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let fan = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&fan) {
            Ok(fan)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "fan sensor",
                index,
            })
        }
    }
}

#[cfg(feature = "writable")]
impl FanSensor for ReadWriteFan {}
#[cfg(feature = "writable")]
impl Faulty for ReadWriteFan {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteFan {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyFan> for ReadWriteFan {
    type Error = SensorError;

    fn try_from(value: ReadOnlyFan) -> Result<Self, Self::Error> {
        let read_write = ReadWriteFan {
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
