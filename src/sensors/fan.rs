//! Module containing the fan sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::{AngularVelocity, FanDivisor, Raw};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only fan sensor.
pub trait FanSensor:
    Sensor<Value = AngularVelocity>
    + shared_subfunctions::Enable
    + shared_subfunctions::Input
    + shared_subfunctions::Min
    + shared_subfunctions::Max
    + shared_subfunctions::Faulty
    + shared_subfunctions::Alarm
    + shared_subfunctions::MinAlarm
    + shared_subfunctions::MaxAlarm
    + shared_subfunctions::Beep
    + std::fmt::Debug
{
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

impl Sensor for FanSensorStruct {
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

impl Parseable for FanSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let fan = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(fan, SensorSubFunctionType::Input)
    }
}

impl shared_subfunctions::Input for FanSensorStruct {}
impl shared_subfunctions::Enable for FanSensorStruct {}
impl shared_subfunctions::Min for FanSensorStruct {}
impl shared_subfunctions::Max for FanSensorStruct {}
impl shared_subfunctions::Faulty for FanSensorStruct {}
impl shared_subfunctions::Alarm for FanSensorStruct {}
impl shared_subfunctions::Beep for FanSensorStruct {}
impl FanSensor for FanSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for FanSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write fan sensor.
pub trait WriteableFanSensor:
    FanSensor
    + WriteableSensor
    + shared_subfunctions::WriteableEnable
    + shared_subfunctions::WriteableMin
    + shared_subfunctions::WriteableMax
    + shared_subfunctions::WriteableBeep
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
