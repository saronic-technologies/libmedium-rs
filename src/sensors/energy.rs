//! Module containing the energy sensors and their related functionality.

use super::*;
use crate::hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Energy;

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only energy sensor.
pub trait EnergySensor: Sensor<Value = Energy> + std::fmt::Debug {
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

        inspect_sensor(energy, SensorSubFunctionType::Input)
    }

    fn prefix() -> &'static str {
        "energy"
    }
}

impl EnergySensor for EnergySensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for EnergySensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write energy sensor.
pub trait WriteableEnergySensor: EnergySensor + WriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableEnergySensor for EnergySensorStruct {}
