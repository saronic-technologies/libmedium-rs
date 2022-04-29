//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Voltage;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only voltage sensor.
pub trait VoltageSensor:
    Sensor<Value = Voltage>
    + shared_subfunctions::Enable
    + shared_subfunctions::Input
    + shared_subfunctions::Min
    + shared_subfunctions::Max
    + shared_subfunctions::LowCrit
    + shared_subfunctions::Crit
    + shared_subfunctions::Average
    + shared_subfunctions::Lowest
    + shared_subfunctions::Highest
    + shared_subfunctions::Alarm
    + shared_subfunctions::MinAlarm
    + shared_subfunctions::MaxAlarm
    + shared_subfunctions::CritAlarm
    + shared_subfunctions::LowCritAlarm
    + shared_subfunctions::Beep
    + std::fmt::Debug
{
}

/// Struct that represents a read only voltage sensor.
#[derive(Debug, Clone)]
pub(crate) struct VoltageSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for VoltageSensorStruct {
    type Value = Voltage;

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

impl Parseable for VoltageSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(volt, SensorSubFunctionType::Input)
    }
}

impl shared_subfunctions::Enable for VoltageSensorStruct {}
impl shared_subfunctions::Input for VoltageSensorStruct {}
impl shared_subfunctions::Min for VoltageSensorStruct {}
impl shared_subfunctions::Max for VoltageSensorStruct {}
impl shared_subfunctions::LowCrit for VoltageSensorStruct {}
impl shared_subfunctions::Crit for VoltageSensorStruct {}
impl shared_subfunctions::Average for VoltageSensorStruct {}
impl shared_subfunctions::Lowest for VoltageSensorStruct {}
impl shared_subfunctions::Highest for VoltageSensorStruct {}
impl shared_subfunctions::Alarm for VoltageSensorStruct {}
impl shared_subfunctions::Beep for VoltageSensorStruct {}
impl VoltageSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write voltage sensor.
pub trait WriteableVoltageSensor:
    VoltageSensor
    + WriteableSensor
    + shared_subfunctions::WriteableEnable
    + shared_subfunctions::WriteableMin
    + shared_subfunctions::WriteableMax
    + shared_subfunctions::WriteableCrit
    + shared_subfunctions::WriteableLowCrit
    + shared_subfunctions::WriteableBeep
{
}

#[cfg(feature = "writeable")]
impl WriteableVoltageSensor for VoltageSensorStruct {}
