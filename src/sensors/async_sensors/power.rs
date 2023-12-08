//! Module containing the power sensors and their related functionality.

use super::*;

use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{AsyncParseable, Result as ParsingResult};
use crate::units::{Power, Ratio, Raw};

use std::time::Duration;

#[async_trait]
/// Helper trait that sums up all functionality of a read-only power sensor.
pub trait AsyncPowerSensor: AsyncSensor<Value = Power> + std::fmt::Debug {
    /// Reads the accuracy subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_accuracy(&self) -> Result<Ratio> {
        let raw = self.read_raw(SensorSubFunctionType::Accuracy).await?;
        Ratio::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_cap(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::Cap).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_cap_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMax).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_cap_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMin).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_cap_hyst(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapHyst).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_interval(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageInterval).await?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_interval_max(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMax).await?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_interval_min(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMin).await?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_highest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_highest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageHighest).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_lowest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_lowest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageLowest).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMax).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_average_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMin).await?;
        Power::from_raw(&raw).map_err(Error::from)
    }

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

    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_average(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Average).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the crit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CritAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the cap subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_cap_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CapAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only power sensor.
#[derive(Debug, Clone)]
pub(crate) struct PowerSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for PowerSensorStruct {
    type Value = Power;

    fn base(&self) -> &'static str {
        "power"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[async_trait]
impl AsyncParseable for PowerSensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(power, SensorSubFunctionType::Input).await
    }

    fn prefix() -> &'static str {
        "power"
    }
}

impl AsyncPowerSensor for PowerSensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for PowerSensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write power sensor.
pub trait AsyncWriteablePowerSensor: AsyncPowerSensor + AsyncWriteableSensor {
    /// Converts cap and writes it to the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_cap(&self, cap: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Cap, &cap.to_raw()).await
    }

    /// Converts cap_hyst and writes it to the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_cap_hyst(&self, cap_hyst: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::CapHyst, &cap_hyst.to_raw()).await
    }

    /// Converts interval and writes it to the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_average_interval(&self, interval: Duration) -> Result<()> {
        self.write_raw(SensorSubFunctionType::AverageInterval, &interval.to_raw()).await
    }

    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw()).await
    }

    /// Writes this sensor's max value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_max(&self, max: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw()).await
    }

    /// Writes this sensor's crit value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_crit(&self, crit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw()).await
    }

    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw()).await
    }
}

#[cfg(feature = "writeable")]
impl AsyncWriteablePowerSensor for PowerSensorStruct {}
