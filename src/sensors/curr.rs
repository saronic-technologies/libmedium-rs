//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Current;

#[cfg(feature = "writeable")]
use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only current sensor.
pub trait CurrentSensor:
    Sensor<Value = Current>
    + Enable
    + Input
    + Min
    + Max
    + LowCrit
    + Crit
    + Average
    + Lowest
    + Highest
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

        inspect_sensor(curr)
    }
}

impl Enable for CurrentSensorStruct {}
impl Input for CurrentSensorStruct {}
impl Min for CurrentSensorStruct {}
impl Max for CurrentSensorStruct {}
impl Crit for CurrentSensorStruct {}
impl LowCrit for CurrentSensorStruct {}
impl Average for CurrentSensorStruct {}
impl Lowest for CurrentSensorStruct {}
impl Highest for CurrentSensorStruct {}
impl CurrentSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write current sensor.
pub trait WriteableCurrentSensor:
    CurrentSensor
    + WriteableSensor
    + WriteableEnable
    + WriteableMin
    + WriteableMax
    + WriteableCrit
    + WriteableLowCrit
{
}

#[cfg(feature = "writeable")]
impl WriteableCurrentSensor for CurrentSensorStruct {}
