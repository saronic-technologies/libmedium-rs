//! Module containing the intrusion sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only intrusion sensor.
pub trait IntrusionSensor:
    Sensor<Value = bool> + shared_subfunctions::Alarm + shared_subfunctions::Beep + std::fmt::Debug
{
}

/// Struct that represents a read only intrusion sensor.
#[derive(Debug, Clone)]
pub(crate) struct IntrusionSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for IntrusionSensorStruct {
    type Value = bool;

    fn base(&self) -> &'static str {
        "intrusion"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for IntrusionSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let intrusion = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(intrusion, SensorSubFunctionType::Alarm)
    }
}

impl shared_subfunctions::Alarm for IntrusionSensorStruct {}
impl shared_subfunctions::Beep for IntrusionSensorStruct {}
impl IntrusionSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write intrusion sensor.
pub trait WriteableIntrusionSensor:
    IntrusionSensor + WriteableSensor + shared_subfunctions::WriteableBeep
{
}

#[cfg(feature = "writeable")]
impl WriteableIntrusionSensor for IntrusionSensorStruct {}
