//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{AsyncParseable, Result as ParsingResult};
use crate::units::Ratio;

use std::path::{Path, PathBuf};

#[async_trait]
/// Helper trait that sums up all functionality of a read-only humidity sensor.
pub trait AsyncHumiditySensor: AsyncSensor<Value = Ratio> + std::fmt::Debug {
    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the input subfunction of this sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    async fn read_input(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Input).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only humidity sensor.
#[derive(Debug, Clone)]
pub(crate) struct HumiditySensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for HumiditySensorStruct {
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

#[async_trait]
impl AsyncParseable for HumiditySensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity, SensorSubFunctionType::Input).await
    }

    fn prefix() -> &'static str {
        "humidity"
    }
}

impl AsyncHumiditySensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for HumiditySensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write humidity sensor.
pub trait AsyncWriteableHumiditySensor: AsyncHumiditySensor + AsyncWriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
            .await
    }
}

#[cfg(feature = "writeable")]
impl AsyncWriteableHumiditySensor for HumiditySensorStruct {}
