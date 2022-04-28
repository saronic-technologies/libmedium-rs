//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Voltage;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only voltage sensor.
pub trait VoltageSensor:
    Sensor<Value = Voltage>
    + subfunctions::Enable
    + subfunctions::Input
    + subfunctions::Min
    + subfunctions::Max
    + subfunctions::LowCrit
    + subfunctions::Crit
    + subfunctions::Average
    + subfunctions::Lowest
    + subfunctions::Highest
    + subfunctions::Alarm
    + subfunctions::MinAlarm
    + subfunctions::MaxAlarm
    + subfunctions::CritAlarm
    + subfunctions::LowCritAlarm
    + subfunctions::Beep
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

impl subfunctions::Enable for VoltageSensorStruct {}
impl subfunctions::Input for VoltageSensorStruct {}
impl subfunctions::Min for VoltageSensorStruct {}
impl subfunctions::Max for VoltageSensorStruct {}
impl subfunctions::LowCrit for VoltageSensorStruct {}
impl subfunctions::Crit for VoltageSensorStruct {}
impl subfunctions::Average for VoltageSensorStruct {}
impl subfunctions::Lowest for VoltageSensorStruct {}
impl subfunctions::Highest for VoltageSensorStruct {}
impl subfunctions::Alarm for VoltageSensorStruct {}
impl subfunctions::Beep for VoltageSensorStruct {}
impl VoltageSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write voltage sensor.
pub trait WriteableVoltageSensor:
    VoltageSensor
    + WriteableSensor
    + subfunctions::WriteableEnable
    + subfunctions::WriteableMin
    + subfunctions::WriteableMax
    + subfunctions::WriteableCrit
    + subfunctions::WriteableLowCrit
    + subfunctions::WriteableBeep
{
}

#[cfg(feature = "writeable")]
impl WriteableVoltageSensor for VoltageSensorStruct {}
