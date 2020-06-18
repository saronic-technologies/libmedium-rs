//! Module containing the fan sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::{AngularVelocity, FanDivisor, Raw};
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all fan sensors.
pub trait FanSensor: SensorBase {
    /// Reads the target_revs subfunction of this fan sensor.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_target(&self) -> Result<AngularVelocity> {
        let raw = self.read_raw(SensorSubFunctionType::Target)?;
        AngularVelocity::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the div subfunction of this fan sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_div(&self) -> Result<FanDivisor> {
        let raw = self.read_raw(SensorSubFunctionType::Div)?;
        FanDivisor::from_raw(&raw).map_err(Error::from)
    }

    /// Converts target and writes it to this fan's target subfunction.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_target(&self, target: AngularVelocity) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Target, &target.to_raw())
    }

    /// Converts div and writes it to this fan's divisor subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_div(&self, div: FanDivisor) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Div, &div.to_raw())
    }
}

impl<S: FanSensor + Faulty> Sensor<AngularVelocity> for S {
    /// Reads the input subfunction of this fan sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_input(&self) -> Result<AngularVelocity> {
        if self.read_faulty().unwrap_or(false) {
            return Err(Error::FaultySensor);
        }

        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        AngularVelocity::from_raw(&raw).map_err(Error::from)
    }
}

impl<S: FanSensor> Min<AngularVelocity> for S {}
impl<S: FanSensor> Max<AngularVelocity> for S {}

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

        inspect_sensor(fan)
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

        inspect_sensor(fan)
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
    type Error = Error;

    fn try_from(value: ReadOnlyFan) -> std::result::Result<Self, Self::Error> {
        let read_write = ReadWriteFan {
            hwmon_path: value.hwmon_path,
            index: value.index,
        };

        if read_write.supported_write_sub_functions().is_empty() {
            return Err(Error::InsufficientRights {
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
