//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Voltage;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only voltage sensor.
pub trait VoltageSensor:
    Sensor<Value = Voltage>
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

        inspect_sensor(volt)
    }
}

impl Enable for VoltageSensorStruct {}
impl Input for VoltageSensorStruct {}
impl Min for VoltageSensorStruct {}
impl Max for VoltageSensorStruct {}
impl LowCrit for VoltageSensorStruct {}
impl Crit for VoltageSensorStruct {}
impl Average for VoltageSensorStruct {}
impl Lowest for VoltageSensorStruct {}
impl Highest for VoltageSensorStruct {}
impl VoltageSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write voltage sensor.
pub trait WriteableVoltageSensor:
    VoltageSensor
    + WriteableSensor
    + WriteableEnable
    + WriteableMin
    + WriteableMax
    + WriteableCrit
    + WriteableLowCrit
{
}

#[cfg(feature = "writeable")]
impl WriteableVoltageSensor for VoltageSensorStruct {}
