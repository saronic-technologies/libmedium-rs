//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Current;
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writeable")]
use std::path::{Path, PathBuf};

/// Helper trait that sums up all the functionality of a read-only current sensor.
pub trait CurrentSensor:
    SensorBase
    + Enable
    + Sensor<Current>
    + Min<Current>
    + Max<Current>
    + LowCrit<Current>
    + Crit<Current>
    + Average<Current>
    + Lowest<Current>
    + Highest<Current>
    + std::fmt::Debug
{
}

#[derive(Debug, Clone)]
pub(crate) struct CurrentSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for CurrentSensorStruct {
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
impl Sensor<Current> for CurrentSensorStruct {}
impl Min<Current> for CurrentSensorStruct {}
impl Max<Current> for CurrentSensorStruct {}
impl Crit<Current> for CurrentSensorStruct {}
impl LowCrit<Current> for CurrentSensorStruct {}
impl Average<Current> for CurrentSensorStruct {}
impl Lowest<Current> for CurrentSensorStruct {}
impl Highest<Current> for CurrentSensorStruct {}
impl CurrentSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensorBase for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all the functionality of a read-write current sensor.
pub trait WriteableCurrentSensor:
    CurrentSensor
    + WriteableSensorBase
    + WriteableEnable
    + WriteableMin<Current>
    + WriteableMax<Current>
    + WriteableCrit<Current>
    + WriteableLowCrit<Current>
{
}

#[cfg(feature = "writeable")]
impl WriteableCurrentSensor for CurrentSensorStruct {}
