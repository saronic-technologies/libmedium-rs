//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::sync_hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::Current;

#[cfg(feature = "writeable")]
use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only current sensor.
pub trait CurrentSensor: Sensor<Value = Current> + std::fmt::Debug {
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

    /// Reads this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_min(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Min)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_max(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Max)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lcrit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_crit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Crit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_average(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Average)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the min subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_min_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MinAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the max subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_max_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MaxAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the crit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CritAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the lcrit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_lcrit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CurrentSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for CurrentSensorStruct {
    type Value = Current;

    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for CurrentSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(curr, SensorSubFunctionType::Input)
    }

    fn prefix() -> &'static str {
        "curr"
    }
}

impl CurrentSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for CurrentSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write current sensor.
pub trait WriteableCurrentSensor: CurrentSensor + WriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }

    /// Writes this sensor's min value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_min(&self, min: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Min, &min.to_raw())
    }

    /// Writes this sensor's max value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_max(&self, max: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw())
    }

    /// Writes this sensor's crit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_crit(&self, crit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw())
    }

    /// Writes this sensor's lcrit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_lcrit(&self, lcrit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
    }

    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableCurrentSensor for CurrentSensorStruct {}
