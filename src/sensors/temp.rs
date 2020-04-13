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

//! Module containing the temp sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::{Parseable, ParsingResult};

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, Div, Mul};
use std::path::{Path, PathBuf};

/// Struct that represents a temperature.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Temperature(i32);

impl Temperature {
    /// Create a Temperature struct from a value measuring degrees celsius.
    pub fn from_degrees_celsius(degrees: impl Into<f64>) -> Self {
        Self((degrees.into() * 1000.0) as i32)
    }

    /// Create a Temperature struct from a value measuring millidegrees celsius.
    pub fn from_millidegrees_celsius(millidegrees: impl Into<i32>) -> Self {
        Self(millidegrees.into())
    }

    /// Create a Temperature struct from a value measuring degrees fahrenheit.
    pub fn from_degrees_fahrenheit(degrees: impl Into<f64>) -> Self {
        Self::from_degrees_celsius((degrees.into() - 32.0) / 1.8)
    }

    /// Returns this struct's value as degrees celsius.
    pub fn as_degrees_celsius(self) -> f64 {
        f64::from(self.0) / 1000.0
    }

    /// Returns this struct's value as millidegrees celsius.
    pub fn as_millidegrees_celsius(self) -> i32 {
        self.0
    }

    /// Returns this struct's value as degrees fahrenheit.
    pub fn as_degrees_fahrenheit(self) -> f64 {
        self.as_degrees_celsius() * 1.8 + 32.0
    }
}

impl Raw for Temperature {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<i32>()
            .map(Temperature::from_millidegrees_celsius)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> String {
        self.as_millidegrees_celsius().to_string()
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.as_degrees_celsius())
    }
}

impl Eq for Temperature {}

impl Ord for Temperature {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for Temperature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Temperature(self.0 + other.0)
    }
}

impl<T: Into<i32>> Mul<T> for Temperature {
    type Output = Self;

    fn mul(self, other: T) -> Temperature {
        Temperature(self.0 * other.into())
    }
}

impl<T: Into<i32>> Div<T> for Temperature {
    type Output = Self;

    fn div(self, other: T) -> Temperature {
        Temperature(self.0 / other.into())
    }
}

/// Enum that represents the different temp sensor types.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum TempType {
    CpuEmbeddedDiode,
    Transistor,
    ThermalDiode,
    Thermistor,
    AmdAmdsi,
    IntelPeci,
}

impl Raw for TempType {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "1" => Ok(TempType::CpuEmbeddedDiode),
            "2" => Ok(TempType::Transistor),
            "3" => Ok(TempType::ThermalDiode),
            "4" => Ok(TempType::Thermistor),
            "5" => Ok(TempType::AmdAmdsi),
            "6" => Ok(TempType::IntelPeci),
            _ => Err(RawError::from(raw)),
        }
    }

    fn to_raw(&self) -> String {
        match self {
            TempType::CpuEmbeddedDiode => String::from("1"),
            TempType::Transistor => String::from("2"),
            TempType::ThermalDiode => String::from("3"),
            TempType::Thermistor => String::from("4"),
            TempType::AmdAmdsi => String::from("5"),
            TempType::IntelPeci => String::from("6"),
        }
    }
}

/// Trait implemented by all temp sensors.
pub trait TempSensor: SensorBase {
    /// Reads the type subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_type(&self) -> SensorResult<TempType> {
        let raw = self.read_raw(SensorSubFunctionType::Type)?;
        TempType::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the offset subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_offset(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Offset)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the max_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_max_hyst(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MaxHyst)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the min_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_min_hyst(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MinHyst)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the crit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_crit_hyst(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::CritHyst)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the emergency subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_emergency(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Emergency)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the emergency_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_emergency_hyst(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::EmergencyHyst)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads the lcrit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_lcrit_hyst(&self) -> SensorResult<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritHyst)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }

    /// Converts offset and writes it to this temp's offset subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_offset(&self, offset: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Offset, &offset.to_raw().to_raw())
    }

    /// Converts max_hyst and writes it to this temp's max_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_max_hyst(&self, max_hyst: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::MaxHyst, &max_hyst.to_raw())
    }

    /// Converts min_hyst and writes it to this temp's min_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_min_hyst(&self, min_hyst: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::MinHyst, &min_hyst.to_raw())
    }

    /// Converts crit_hyst and writes it to this temp's crit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_crit_hyst(&self, crit_hyst: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::CritHyst, &crit_hyst.to_raw())
    }

    /// Converts emergency and writes it to this temp's emergency subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_emergency(&self, emergency: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Emergency, &emergency.to_raw())
    }

    /// Converts emergency_hyst and writes it to this temp's emergency_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_emergency_hyst(&self, emergency_hyst: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(
            SensorSubFunctionType::EmergencyHyst,
            &emergency_hyst.to_raw(),
        )
    }

    /// Converts lcrit_hyst and writes it to this temp's lcrit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_lcrit_hyst(&self, lcrit_hyst: Temperature) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::LowCritHyst, &lcrit_hyst.to_raw())
    }
}

impl<S: TempSensor + Faulty> Sensor<Temperature> for S {
    /// Reads the input subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> SensorResult<Temperature> {
        if self.read_faulty().unwrap_or(false) {
            return Err(SensorError::FaultySensor);
        }

        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Temperature::from_raw(&raw).map_err(SensorError::from)
    }
}

impl<S: TempSensor> Min<Temperature> for S {}
impl<S: TempSensor> Max<Temperature> for S {}
impl<S: TempSensor> Crit<Temperature> for S {}
impl<S: TempSensor> LowCrit<Temperature> for S {}

/// Struct that represents a read only temp sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyTemp {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyTemp {
    fn base(&self) -> &'static str {
        "temp"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyTemp {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&temp) {
            Ok(temp)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "temp sensor",
                index,
            })
        }
    }
}

impl TempSensor for ReadOnlyTemp {}
impl Faulty for ReadOnlyTemp {}

#[cfg(feature = "writable")]
impl From<ReadWriteTemp> for ReadOnlyTemp {
    fn from(write_temp: ReadWriteTemp) -> ReadOnlyTemp {
        ReadOnlyTemp {
            hwmon_path: write_temp.hwmon_path,
            index: write_temp.index,
        }
    }
}

/// Struct that represents a read/write temp sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteTemp {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteTemp {
    fn base(&self) -> &'static str {
        "temp"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteTemp {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        if sensor_valid(&temp) {
            Ok(temp)
        } else {
            Err(ParsingError::SensorCreationError {
                sensor_type: "temp sensor",
                index,
            })
        }
    }
}

#[cfg(feature = "writable")]
impl TempSensor for ReadWriteTemp {}
#[cfg(feature = "writable")]
impl Faulty for ReadWriteTemp {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteTemp {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyTemp> for ReadWriteTemp {
    type Error = SensorError;

    fn try_from(value: ReadOnlyTemp) -> Result<Self, Self::Error> {
        let read_write = ReadWriteTemp {
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
