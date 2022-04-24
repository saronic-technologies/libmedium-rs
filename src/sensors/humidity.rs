//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Ratio;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only humidity sensor.
pub trait HumiditySensor: Sensor<Value = Ratio> + Enable + Input + std::fmt::Debug {}

/// Struct that represents a read only humidity sensor.
#[derive(Debug, Clone)]
pub(crate) struct HumiditySensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for HumiditySensorStruct {
    type Value = Ratio;

    fn base(&self) -> &'static str {
        "humidity"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for HumiditySensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity, SensorSubFunctionType::Input)
    }
}

impl Enable for HumiditySensorStruct {}
impl Input for HumiditySensorStruct {}
impl HumiditySensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write humidity sensor.
pub trait WriteableHumiditySensor: HumiditySensor + WriteableSensor + WriteableEnable {}

#[cfg(feature = "writeable")]
impl WriteableHumiditySensor for HumiditySensorStruct {}
