//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::sync_hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Ratio;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only humidity sensor.
pub trait HumiditySensor: Sensor<Value = Ratio> + std::fmt::Debug {
    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the input subfunction of this sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

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

    fn prefix() -> &'static str {
        "humidity"
    }
}

impl HumiditySensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write humidity sensor.
pub trait WriteableHumiditySensor: HumiditySensor + WriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableHumiditySensor for HumiditySensorStruct {}
