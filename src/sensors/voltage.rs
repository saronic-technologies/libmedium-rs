//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Voltage;
use crate::{Parseable, ParsingResult};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all the functionality of a read-only voltage sensor.
pub trait VoltageSensor:
    SensorBase
    + Enable
    + Sensor<Voltage>
    + Min<Voltage>
    + Max<Voltage>
    + LowCrit<Voltage>
    + Crit<Voltage>
    + Average<Voltage>
    + Lowest<Voltage>
    + Highest<Voltage>
    + std::fmt::Debug
{
}

/// Struct that represents a read only voltage sensor.
#[derive(Debug, Clone)]
pub(crate) struct VoltageSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for VoltageSensorStruct {
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
impl Sensor<Voltage> for VoltageSensorStruct {}
impl Min<Voltage> for VoltageSensorStruct {}
impl Max<Voltage> for VoltageSensorStruct {}
impl LowCrit<Voltage> for VoltageSensorStruct {}
impl Crit<Voltage> for VoltageSensorStruct {}
impl Average<Voltage> for VoltageSensorStruct {}
impl Lowest<Voltage> for VoltageSensorStruct {}
impl Highest<Voltage> for VoltageSensorStruct {}
impl VoltageSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensorBase for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all the functionality of a read-write voltage sensor.
pub trait WriteableVoltageSensor:
    VoltageSensor
    + WriteableSensorBase
    + WriteableEnable
    + WriteableMin<Voltage>
    + WriteableMax<Voltage>
    + WriteableCrit<Voltage>
    + WriteableLowCrit<Voltage>
{
}

#[cfg(feature = "writeable")]
impl WriteableVoltageSensor for VoltageSensorStruct {}
