//! Module containing the fan sensors and their related functionality.

use super::*;
use crate::hwmon::sync_hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::{AngularVelocity, FanDivisor, Raw};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only fan sensor.
pub trait FanSensor: Sensor<Value = AngularVelocity> + std::fmt::Debug {
    /// Reads the target_revs subfunction of this fan sensor.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_target(&self) -> Result<AngularVelocity> {
        let raw = self.read_raw(SensorSubFunctionType::Target)?;
        AngularVelocity::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the div subfunction of this fan sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_div(&self) -> Result<FanDivisor> {
        let raw = self.read_raw(SensorSubFunctionType::Div)?;
        FanDivisor::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the input subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> Result<Self::Value> {
        if self.read_faulty().unwrap_or(false) {
            return Err(Error::FaultySensor);
        }

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

    /// Reads whether this sensor is faulty or not.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_faulty(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Fault)?;
        bool::from_raw(&raw).map_err(Error::from)
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

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only fan sensor.
#[derive(Debug, Clone)]
pub(crate) struct FanSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for FanSensorStruct {
    type Value = AngularVelocity;

    fn base(&self) -> &'static str {
        "fan"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for FanSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let fan = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(fan, SensorSubFunctionType::Input)
    }

    fn prefix() -> &'static str {
        "fan"
    }
}

impl FanSensor for FanSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for FanSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write fan sensor.
pub trait WriteableFanSensor: FanSensor + WriteableSensor {
    /// Converts target and writes it to this fan's target subfunction.
    ///
    /// Only makes sense if the chip supports closed-loop fan speed control based on the measured fan speed.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_target(&self, target: AngularVelocity) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Target, &target.to_raw())
    }

    /// Converts div and writes it to this fan's divisor subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_div(&self, div: FanDivisor) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Div, &div.to_raw())
    }

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

    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableFanSensor for FanSensorStruct {}
