//! Module containing the temp sensors and their related functionality.

use super::*;
use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{AsyncParseable, Result as ParsingResult};
use crate::units::{Raw, TempType, Temperature};

use std::path::{Path, PathBuf};

#[async_trait]
/// Helper trait that sums up all functionality of a read-only temp sensor.
pub trait AsyncTempSensor: AsyncSensor<Value = Temperature> + std::fmt::Debug {
    /// Reads the type subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_type(&self) -> Result<TempType> {
        let raw = self.read_raw(SensorSubFunctionType::Type).await?;
        TempType::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the offset subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_offset(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Offset).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the max_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_max_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MaxHyst).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the min_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_min_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MinHyst).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the crit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_crit_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::CritHyst).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the emergency subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_emergency(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Emergency).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the emergency_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_emergency_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::EmergencyHyst).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_lcrit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit).await?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the lcrit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_lcrit_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritHyst).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the input subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    async fn read_input(&self) -> Result<Temperature> {
        if self.read_faulty().await.unwrap_or(false) {
            return Err(Error::FaultySensor);
        }

        let raw = self.read_raw(SensorSubFunctionType::Input).await?;
        Temperature::from_raw(&raw).map_err(Error::from)
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

    /// Reads whether this sensor is faulty or not.
    /// Returns an error, if this sensor doesn't support the feature.
    async fn read_faulty(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Fault).await?;
        bool::from_raw(&raw).map_err(Error::from)
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

    /// Reads whether or not an alarm condition exists for the emergency subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_emergency_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::EmergencyAlarm).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep).await?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only temp sensor.
#[derive(Debug, Clone)]
pub(crate) struct TempSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for TempSensorStruct {
    type Value = Temperature;

    fn base(&self) -> &'static str {
        "temp"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[async_trait]
impl AsyncParseable for TempSensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(temp, SensorSubFunctionType::Input).await
    }

    fn prefix() -> &'static str {
        "temp"
    }
}

impl AsyncTempSensor for TempSensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for TempSensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write temp sensor.
pub trait AsyncWriteableTempSensor: AsyncTempSensor + AsyncWriteableSensor {
    /// Converts offset and writes it to this temp's offset subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_offset(&self, offset: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Offset, &offset.to_raw())
            .await
    }

    /// Converts max_hyst and writes it to this temp's max_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_max_hyst(&self, max_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::MaxHyst, &max_hyst.to_raw())
            .await
    }

    /// Converts min_hyst and writes it to this temp's min_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_min_hyst(&self, min_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::MinHyst, &min_hyst.to_raw())
            .await
    }

    /// Converts crit_hyst and writes it to this temp's crit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_crit_hyst(&self, crit_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::CritHyst, &crit_hyst.to_raw())
            .await
    }

    /// Converts emergency and writes it to this temp's emergency subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_emergency(&self, emergency: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Emergency, &emergency.to_raw())
            .await
    }

    /// Converts emergency_hyst and writes it to this temp's emergency_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_emergency_hyst(&self, emergency_hyst: Temperature) -> Result<()> {
        self.write_raw(
            SensorSubFunctionType::EmergencyHyst,
            &emergency_hyst.to_raw(),
        )
        .await
    }

    /// Writes this sensor's lcrit value.
    /// Returns an error, if the sensor doesn't support the feature.
    async fn write_lcrit(&self, lcrit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
            .await
    }

    /// Converts lcrit_hyst and writes it to this temp's lcrit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_lcrit_hyst(&self, lcrit_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCritHyst, &lcrit_hyst.to_raw())
            .await
    }

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
impl AsyncWriteableTempSensor for TempSensorStruct {}
