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
use crate::Parseable;

use std::cmp::Ordering;
use std::fmt;
use std::path::Path;

/// Struct that represents a pwm value between 0 and 255.
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub struct Pwm(u8);

impl Pwm {
    /// Construct a new Pwm struct from a pwm value between 0 and 255.
    pub fn from_u8(u8: u8) -> Self {
        Self(u8)
    }

    /// Returns this struct's pwm value between 0 and 255.
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Construct a new Pwm struct from a pwm value in percent.
    pub fn from_percent(percent: f64) -> Self {
        Self((percent * 2.55) as u8)
    }

    /// Returns this struct's pwm value in percent.
    pub fn as_percent(self) -> f64 {
        f64::from(self.0) / 2.55
    }
}

impl Raw for Pwm {
    fn to_raw(&self) -> String {
        self.0.to_string()
    }

    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.parse::<u8>()
            .map(Self::from_u8)
            .map_err(|_| RawError::InvalidRawString(raw.to_string()))
    }
}

impl Ord for Pwm {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for Pwm {}

impl fmt::Display for Pwm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percent())
    }
}

/// Enum that represents the control states a pwm can be in.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum PwmEnable {
    FullSpeed,
    ManualControl,
    BiosControl,
}

impl Raw for PwmEnable {
    fn to_raw(&self) -> String {
        match self {
            PwmEnable::FullSpeed => String::from("0"),
            PwmEnable::ManualControl => String::from("1"),
            PwmEnable::BiosControl => String::from("2"),
        }
    }

    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "0" => Ok(PwmEnable::FullSpeed),
            "1" => Ok(PwmEnable::ManualControl),
            "2" => Ok(PwmEnable::BiosControl),
            raw => Err(RawError::InvalidRawString(raw.to_string())),
        }
    }
}

impl Default for PwmEnable {
    fn default() -> PwmEnable {
        PwmEnable::BiosControl
    }
}

/// Struct that represents the modes by which a fan's speed can be regulated.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Hash)]
pub enum PwmMode {
    DC,
    PWM,
    Automatic,
}

impl Raw for PwmMode {
    fn to_raw(&self) -> String {
        match self {
            PwmMode::DC => String::from("0"),
            PwmMode::PWM => String::from("1"),
            PwmMode::Automatic => String::from("2"),
        }
    }

    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "0" => Ok(PwmMode::DC),
            "1" => Ok(PwmMode::PWM),
            "2" => Ok(PwmMode::Automatic),
            raw => Err(RawError::InvalidRawString(raw.to_string())),
        }
    }
}

impl Default for PwmMode {
    fn default() -> PwmMode {
        PwmMode::Automatic
    }
}

/// Struct that represents a pwm's base frequency.
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Frequency(u32);

impl Frequency {
    /// Return this Frequency's value in hertz.
    pub fn to_hz(self) -> u32 {
        self.0
    }

    /// Create a Frequency struct from a value measuring hertz.
    pub fn from_hz(hz: u32) -> Self {
        Self(hz)
    }
}

impl Raw for Frequency {
    fn to_raw(&self) -> String {
        self.0.to_string()
    }

    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<u32>()
            .map(Frequency)
            .map_err(|_| RawError::from(raw))
    }
}

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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&pwm)?;

        Ok(pwm)
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

#[cfg(feature = "writable")]
impl From<&ReadWritePwm> for ReadOnlyPwm {
    fn from(write_pwm: &ReadWritePwm) -> ReadOnlyPwm {
        ReadOnlyPwm {
            hwmon_path: write_pwm.hwmon_path().to_path_buf(),
            index: write_pwm.index(),
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
    type Error = SensorError;

    fn parse(parent: &Self::Parent, index: u16) -> SensorResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        check_sensor(&pwm)?;

        Ok(pwm)
    }
}

#[cfg(feature = "writable")]
impl PwmSensor for ReadWritePwm {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWritePwm {}
