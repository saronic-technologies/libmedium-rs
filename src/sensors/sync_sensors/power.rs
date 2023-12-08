//! Module containing the power sensors and their related functionality.

use super::*;
use crate::hwmon::sync_hwmon::Hwmon;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::{Power, Ratio, Raw};

/// Helper trait that sums up all functionality of a read-only power sensor.
pub trait PowerSensor: Sensor<Value = Power> + std::fmt::Debug {
    /// Reads the accuracy subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_accuracy(&self) -> Result<Ratio> {
        let raw = self.read_raw(SensorSubFunctionType::Accuracy)?;
        Ratio::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::Cap)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMax)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMin)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_hyst(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapHyst)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageInterval)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_max(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMax)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_min(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMin)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_highest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_highest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageHighest)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_lowest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_lowest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageLowest)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMax)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMin)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

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

    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_max(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Max)?;
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

    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the crit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CritAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition exists for the cap subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_cap_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CapAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }

    /// Reads whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_beep(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Beep)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only power sensor.
#[derive(Debug, Clone)]
pub(crate) struct PowerSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for PowerSensorStruct {
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

impl Parseable for PowerSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(power, SensorSubFunctionType::Input)
    }

    fn prefix() -> &'static str {
        "power"
    }
}

impl PowerSensor for PowerSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for PowerSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write power sensor.
pub trait WriteablePowerSensor: PowerSensor + WriteableSensor {
    /// Converts cap and writes it to the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_cap(&self, cap: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Cap, &cap.to_raw())
    }

    /// Converts cap_hyst and writes it to the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_cap_hyst(&self, cap_hyst: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::CapHyst, &cap_hyst.to_raw())
    }

    /// Converts interval and writes it to the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_average_interval(&self, interval: Duration) -> Result<()> {
        self.write_raw(SensorSubFunctionType::AverageInterval, &interval.to_raw())
    }

    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
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

    /// Sets whether or not an alarm condition for the sensor also triggers beeping.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_beep(&self, beep: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Beep, &beep.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteablePowerSensor for PowerSensorStruct {}
