use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all non virtual sensors except for pwm.
/// It contains the functionality to read the alarm subfunction.
pub trait Alarm: Sensor {
    /// Reads whether or not an alarm condition exists for the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Alarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that support the min and alarm shared_subfunctions.
/// It contains the functionality to read the min_alarm subfunction.
pub trait MinAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the min subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_min_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MinAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

impl<T: super::Min + Alarm> MinAlarm for T {}

/// Trait implemented by all sensors that support the max and alarm shared_subfunctions.
/// It contains the functionality to read the max_alarm subfunction.
pub trait MaxAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the max subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_max_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::MaxAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

impl<T: super::Max + Alarm> MaxAlarm for T {}

/// Trait implemented by all sensors that support the crit and alarm shared_subfunctions.
/// It contains the functionality to read the crit_alarm subfunction.
pub trait CritAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the crit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CritAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

impl<T: super::Crit + Alarm> CritAlarm for T {}

/// Trait implemented by all sensors that support the lcrit and alarm shared_subfunctions.
/// It contains the functionality to read the crit_alarm subfunction.
pub trait LowCritAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the lcrit subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_low_crit_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

impl<T: super::LowCrit + Alarm> LowCritAlarm for T {}

/// Trait implemented by all sensors that support the cap and alarm shared_subfunctions.
/// It contains the functionality to read the crit_alarm subfunction.
pub trait CapAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the cap subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_cap_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::CapAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that support the emergency and alarm shared_subfunctions.
/// It contains the functionality to read the emergency_alarm subfunction.
pub trait EmergencyAlarm: Sensor {
    /// Reads whether or not an alarm condition exists for the emergency subfunction of the sensor.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_emergency_alarm(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::EmergencyAlarm)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}
