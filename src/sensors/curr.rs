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

impl shared_subfunctions::Enable for CurrentSensorStruct {}
impl shared_subfunctions::Input for CurrentSensorStruct {}
impl shared_subfunctions::Min for CurrentSensorStruct {}
impl shared_subfunctions::Max for CurrentSensorStruct {}
impl shared_subfunctions::Crit for CurrentSensorStruct {}
impl shared_subfunctions::LowCrit for CurrentSensorStruct {}
impl shared_subfunctions::Average for CurrentSensorStruct {}
impl shared_subfunctions::Lowest for CurrentSensorStruct {}
impl shared_subfunctions::Highest for CurrentSensorStruct {}
impl shared_subfunctions::Alarm for CurrentSensorStruct {}
impl shared_subfunctions::Beep for CurrentSensorStruct {}
impl CurrentSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write current sensor.
pub trait WriteableCurrentSensor:
    CurrentSensor
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
impl WriteableCurrentSensor for CurrentSensorStruct {}
