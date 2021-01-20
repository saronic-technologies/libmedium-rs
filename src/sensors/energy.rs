//! Module containing the energy sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Energy;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only energy sensor.
pub trait EnergySensor: Sensor<Value = Energy> + Enable + Input + std::fmt::Debug + Clone {}

#[derive(Debug, Clone)]
pub(crate) struct EnergySensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for EnergySensorStruct {
    type Value = Energy;

    fn base(&self) -> &'static str {
        "energy"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for EnergySensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(energy)
    }
}

impl Enable for EnergySensorStruct {}
impl Input for EnergySensorStruct {}
impl EnergySensor for EnergySensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for EnergySensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write energy sensor.
pub trait WriteableEnergySensor: EnergySensor + WriteableSensor + WriteableEnable {}

#[cfg(feature = "writeable")]
impl WriteableEnergySensor for EnergySensorStruct {}
