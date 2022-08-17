//! Module containing the intrusion sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only intrusion sensor.
pub trait IntrusionSensor: Sensor<Value = bool> + std::fmt::Debug {
    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
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

    fn prefix() -> &'static str {
        "intrusion"
    }
}

impl IntrusionSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write intrusion sensor.
pub trait WriteableIntrusionSensor: IntrusionSensor + WriteableSensor {
    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableIntrusionSensor for IntrusionSensorStruct {}
