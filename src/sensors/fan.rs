//! Module containing the fan sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::{AngularVelocity, FanDivisor, Raw};
use crate::{Parseable, ParsingResult};

use std::path::{Path, PathBuf};

/// Trait that sums up all the functionality of a read-only fan sensor.
pub trait FanSensor: Enable + Sensor + Min + Max + Faulty + std::fmt::Debug {
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
}

/// Struct that represents a read only fan sensor.
#[derive(Debug, Clone)]
pub(crate) struct FanSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for FanSensorStruct {
    type Value = AngularVelocity;

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

impl Sensor for FanSensorStruct {
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

impl Parseable for FanSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let fan = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(fan)
    }
}

impl Enable for FanSensorStruct {}
impl Min for FanSensorStruct {}
impl Max for FanSensorStruct {}
impl Faulty for FanSensorStruct {}
impl FanSensor for FanSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensorBase for FanSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all the functionality of a read-write fan sensor.
pub trait WriteableFanSensor:
    FanSensor + WriteableSensorBase + WriteableEnable + WriteableMin + WriteableMax
{
    /// Converts target and writes it to this fan's target subfunction.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_target(&self, target: AngularVelocity) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Target, &target.to_raw())
    }

    /// Converts div and writes it to this fan's divisor subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_div(&self, div: FanDivisor) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Div, &div.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableFanSensor for FanSensorStruct {}
