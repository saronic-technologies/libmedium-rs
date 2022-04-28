//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Current;

#[cfg(feature = "writeable")]
use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only current sensor.
pub trait CurrentSensor:
    Sensor<Value = Current>
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

#[derive(Debug, Clone)]
pub(crate) struct CurrentSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for CurrentSensorStruct {
    type Value = Current;

    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for CurrentSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(curr, SensorSubFunctionType::Input)
    }
}

impl subfunctions::Enable for CurrentSensorStruct {}
impl subfunctions::Input for CurrentSensorStruct {}
impl subfunctions::Min for CurrentSensorStruct {}
impl subfunctions::Max for CurrentSensorStruct {}
impl subfunctions::Crit for CurrentSensorStruct {}
impl subfunctions::LowCrit for CurrentSensorStruct {}
impl subfunctions::Average for CurrentSensorStruct {}
impl subfunctions::Lowest for CurrentSensorStruct {}
impl subfunctions::Highest for CurrentSensorStruct {}
impl subfunctions::Alarm for CurrentSensorStruct {}
impl subfunctions::Beep for CurrentSensorStruct {}
impl CurrentSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write current sensor.
pub trait WriteableCurrentSensor:
    CurrentSensor
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
impl WriteableCurrentSensor for CurrentSensorStruct {}
