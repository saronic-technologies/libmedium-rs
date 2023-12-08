//! Module containing the intrusion sensors and their related functionality.

use super::*;
use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{AsyncParseable, Result as ParsingResult};

use std::path::{Path, PathBuf};

#[async_trait]
/// Helper trait that sums up all functionality of a read-only intrusion sensor.
pub trait AsyncIntrusionSensor: AsyncSensor<Value = bool> + std::fmt::Debug {
    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only intrusion sensor.
#[derive(Debug, Clone)]
pub(crate) struct IntrusionSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for IntrusionSensorStruct {
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

#[async_trait]
impl AsyncParseable for IntrusionSensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let intrusion = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(intrusion, SensorSubFunctionType::Alarm).await
    }

    fn prefix() -> &'static str {
        "intrusion"
    }
}

impl AsyncIntrusionSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for IntrusionSensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write intrusion sensor.
pub trait AsyncWriteableIntrusionSensor: AsyncIntrusionSensor + AsyncWriteableSensor {
    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw()).await
    }
}

#[cfg(feature = "writeable")]
impl AsyncWriteableIntrusionSensor for IntrusionSensorStruct {}
