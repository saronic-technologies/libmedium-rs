//! Module containing the async sensors and their functionality.

pub mod curr;
pub mod energy;
pub mod fan;
pub mod humidity;
pub mod intrusion;
pub mod power;
pub mod pwm;
pub mod temp;
pub mod virt;
pub mod voltage;

use super::error::{Error, Result};

use crate::hwmon::async_hwmon::Hwmon;
use crate::parsing::{Error as ParsingError, Result as ParsingResult};
use crate::sensors::SensorSubFunctionType;
use crate::units::Raw;

use async_trait::async_trait;

use tokio::fs::read_to_string;

#[cfg(feature = "writeable")]
use tokio::fs::write;

#[cfg(feature = "writeable")]
use std::collections::HashMap;

use std::path::{Path, PathBuf};

/// Base trait that all sensors must implement.
/// It contains the functionality to get a sensor's name, index or supported subfunctions.
#[async_trait]
pub trait AsyncSensor : Sync {
    /// Type used by the sensor for measurements.
    #[cfg(feature = "uom_units")]
    type Value: Raw;

    /// Type used by the sensor for measurements.
    #[cfg(not(feature = "uom_units"))]
    type Value: Raw + std::fmt::Display;

    /// Returns this sensor's base like "temp" or "fan".
    fn base(&self) -> &'static str;

    /// Returns this sensor's index.
    fn index(&self) -> u16;

    /// Returns this sensor's hwmon's path.
    fn hwmon_path(&self) -> &Path;

    /// Returns a list of all readable subfunction types supported by this sensor.
    fn supported_read_sub_functions(&self) -> Vec<SensorSubFunctionType> {
        SensorSubFunctionType::read_list()
            .filter(|&s| {
                std::fs::OpenOptions::new().read(true).open(self.subfunction_path(s)).map(|_| true).unwrap_or(false)
            })
            .collect()
    }

    /// If this sensor has a label, its contents are returned.
    /// Otherwise a plain sensor descriptor is returned.
    async fn name(&self) -> String {
        self.read_raw(SensorSubFunctionType::Label).await
            .unwrap_or_else(|_| format!("{}{}", self.base(), self.index()))
    }

    /// Reads this sensor's subfunction with the given type and returns its value as a raw string.
    /// You should usually prefer the specialized read functions like read_input, because they
    /// automatically convert the read value to the right type.
    /// Returns an error, if this sensor doesn't support the subtype.
    async fn read_raw(&self, sub_type: SensorSubFunctionType) -> Result<String> {
        let path = self.subfunction_path(sub_type);

        match read_to_string(&path).await {
            Ok(s) => Ok(s.trim().to_string()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(Error::SubtypeNotSupported { sub_type }),
                std::io::ErrorKind::PermissionDenied => Err(Error::InsufficientRights { path }),
                _ => Err(Error::Read { source: e, path }),
            },
        }
    }

    /// Returns the path this sensor's subfunction of the given type would have.
    fn subfunction_path(&self, sub_type: SensorSubFunctionType) -> PathBuf {
        self.hwmon_path().join(format!(
            "{}{}{}",
            self.base(),
            self.index(),
            sub_type.to_suffix()
        ))
    }
}

/// Base trait that all writeable sensors must implement.
#[cfg(feature = "writeable")]
#[async_trait]
pub trait AsyncWriteableSensor: AsyncSensor {
    /// Returns a list of all writeable subfunction types supported by this sensor.
    fn supported_write_sub_functions(&self) -> Vec<SensorSubFunctionType> {
        SensorSubFunctionType::write_list()
            .filter(|&s| {
                std::fs::OpenOptions::new().write(true).open(self.subfunction_path(s)).map(|_| true).unwrap_or(false)
            })
            .collect()
    }

    /// Returns a list of all readable and writeable subfunction types supported by this sensor.
    fn supported_read_write_sub_functions(&self) -> Vec<SensorSubFunctionType> {
        SensorSubFunctionType::read_write_list()
            .iter()
            .copied()
            .filter(|&s| {
                std::fs::OpenOptions::new().read(true).write(true).open(self.subfunction_path(s)).map(|_| true).unwrap_or(false)
            })
            .collect()
    }

    /// Writes the given raw string value to this sensor's subfunction with the given type.
    /// You should usually prefer the specialized write functions like write_enable, because they
    /// ensure that no type mismatches occur.
    /// Returns an error, if this sensor doesn't support the subtype.
    async fn write_raw(&self, sub_type: SensorSubFunctionType, raw_value: &str) -> Result<()> {
        let path = self.subfunction_path(sub_type);

        write(&path, raw_value.as_bytes()).await.map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Error::SubtypeNotSupported { sub_type },
            std::io::ErrorKind::PermissionDenied => Error::InsufficientRights { path },
            _ => Error::Write { source: e, path },
        })
    }

    /// Resets this sensor's history.
    /// Returns an error if this functionality is not supported by the sensor.
    async fn reset_history(&self) -> Result<()> {
        self.write_raw(SensorSubFunctionType::ResetHistory, &true.to_raw()).await
    }

    /// Returns a SensorState struct that represents the state of all writeable shared_subfunctions of this sensor.
    async fn state(&self) -> Result<AsyncSensorState> {
        let mut states = HashMap::new();

        for &sub_type in SensorSubFunctionType::read_write_list() {
            if let Ok(value) = self.read_raw(sub_type).await {
                states.insert(sub_type, value);
            }
        }

        Ok(AsyncSensorState { states })
    }

    /// Writes the given state to this sensor.
    /// Returns an error and writes nothing if the given state contains one or more shared_subfunctions that this sensor does not support.
    async fn write_state(&self, state: &AsyncSensorState) -> Result<()> {
        if let Some(&sub_type) = state
            .states
            .keys()
            .find(|s| !self.supported_write_sub_functions().contains(s))
        {
            return Err(Error::SubtypeNotSupported { sub_type });
        }

        self.write_state_lossy(state).await
    }

    /// Writes the given state to this sensor.
    /// All subfunction types contained in the given state that are not supported by this sensor will be ignored.
    async fn write_state_lossy(&self, state: &AsyncSensorState) -> Result<()> {
        for (&sub_type, raw_value) in &state.states {
            if let Err(e) = self.write_raw(sub_type, &raw_value).await {
                match e {
                    Error::SubtypeNotSupported { .. } => continue,
                    _ => return Err(e),
                }
            }
        }

        Ok(())
    }
}

/// A struct that represents the state of all writeable subfunctions of a sensor.
/// It can be used to reset a sensor to a previous state or copy its settings to another sensor.
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "writeable")]
pub struct AsyncSensorState {
    states: HashMap<SensorSubFunctionType, String>,
}

#[cfg(feature = "writeable")]
impl AsyncSensorState {
    /// Returns a SensorState struct created from the given sensor.
    pub async fn from_sensor(sensor: &impl AsyncWriteableSensor) -> Result<AsyncSensorState> {
        sensor.state().await
    }

    /// Returns all subfunction types that this state contains.
    pub fn sub_types(&self) -> Vec<SensorSubFunctionType> {
        self.states.keys().cloned().collect()
    }
}

async fn inspect_sensor<S: AsyncSensor>(
    sensor: S,
    primary_subfunction: SensorSubFunctionType,
) -> ParsingResult<S> {
    if let Err(e) = sensor.subfunction_path(primary_subfunction).metadata() {
        return Err(ParsingError::sensor(
            e,
            sensor.subfunction_path(primary_subfunction),
        ));
    }

    Ok(sensor)
}