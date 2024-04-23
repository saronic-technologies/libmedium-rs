//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{AsyncParseable, Result as ParsingResult};
use crate::units::Voltage;

use std::path::{Path, PathBuf};

#[async_trait]
/// Helper trait that sums up all functionality of a read-only voltage sensor.
pub trait AsyncVoltageSensor: AsyncSensor<Value = Voltage> + std::fmt::Debug {
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

    /// Reads this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_min(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Min).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_max(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Max).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_crit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Crit).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_lcrit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_average(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Average).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the min subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_min_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MinAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the max subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_max_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MaxAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the crit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CritAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the lcrit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_lcrit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only voltage sensor.
#[derive(Debug, Clone)]
pub(crate) struct VoltageSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for VoltageSensorStruct {
    type Value = Voltage;

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

#[async_trait]
impl AsyncParseable for VoltageSensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(volt, SensorSubFunctionType::Input).await
    }

    fn prefix() -> &'static str {
        "in"
    }
}

impl AsyncVoltageSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for VoltageSensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write voltage sensor.
pub trait AsyncWriteableVoltageSensor: AsyncVoltageSensor + AsyncWriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
            .await
    }

    /// Writes this sensor's min value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_min(&self, min: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Min, &min.to_raw())
            .await
    }

    /// Writes this sensor's max value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_max(&self, max: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw())
            .await
    }

    /// Writes this sensor's lcrit value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_lcrit(&self, lcrit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
            .await
    }

    /// Writes this sensor's crit value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_crit(&self, crit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw())
            .await
    }

    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
            .await
    }
}

#[cfg(feature = "writeable")]
impl AsyncWriteableVoltageSensor for VoltageSensorStruct {}
