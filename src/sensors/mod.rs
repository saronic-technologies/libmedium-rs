//! Module containing the sensors and their functionality.

mod error;
mod subfunction_type;

pub mod curr;
pub mod energy;
pub mod fan;
pub mod humidity;
pub mod intrusion;
pub mod power;
pub mod pwm;
pub mod shared_subfunctions;
pub mod temp;
pub mod virt;
pub mod voltage;

pub use error::Error;
pub use subfunction_type::SensorSubFunctionType;

use crate::hwmon::Hwmon;
use crate::parsing::{Error as ParsingError, Result as ParsingResult};
use crate::units::Raw;
use error::Result;

#[cfg(feature = "writeable")]
use std::{collections::HashMap, fs::write};

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Duration,
};

/// Base trait that all sensors must implement.
/// It contains the functionality to get a sensor's name, index or supported subfunctions.
pub trait Sensor {
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
            .filter(|&s| self.read_raw(s).is_ok())
            .collect()
    }

    /// If this sensor has a label, its contents are returned.
    /// Otherwise a plain sensor descriptor is returned.
    fn name(&self) -> String {
        self.read_raw(SensorSubFunctionType::Label)
            .unwrap_or_else(|_| format!("{}{}", self.base(), self.index()))
    }

    /// Reads this sensor's subfunction with the given type and returns its value as a raw string.
    /// You should usually prefer the specialized read functions like read_input, because they
    /// automatically convert the read value to the right type.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_raw(&self, sub_type: SensorSubFunctionType) -> Result<String> {
        let path = self.subfunction_path(sub_type);

        match read_to_string(&path) {
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
pub trait WriteableSensor: Sensor {
    /// Returns a list of all writeable subfunction types supported by this sensor.
    fn supported_write_sub_functions(&self) -> Vec<SensorSubFunctionType> {
        SensorSubFunctionType::write_list()
            .filter(|&s| {
                self.subfunction_path(s)
                    .metadata()
                    .map(|m| !m.permissions().readonly())
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Writes the given raw string value to this sensor's subfunction with the given type.
    /// You should usually prefer the specialized write functions like write_enable, because they
    /// ensure that no type mismatches occur.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn write_raw(&self, sub_type: SensorSubFunctionType, raw_value: &str) -> Result<()> {
        let path = self.subfunction_path(sub_type);

        write(&path, raw_value.as_bytes()).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Error::SubtypeNotSupported { sub_type },
            std::io::ErrorKind::PermissionDenied => Error::InsufficientRights { path },
            _ => Error::Write { source: e, path },
        })
    }

    /// Resets this sensor's history.
    /// Returns an error if this functionality is not supported by the sensor.
    fn reset_history(&self) -> Result<()> {
        self.write_raw(SensorSubFunctionType::ResetHistory, &true.to_raw())
    }

    /// Returns a SensorState struct that represents the state of all writeable shared_subfunctions of this sensor.
    fn state(&self) -> Result<SensorState> {
        let mut states = HashMap::new();
        let supported_read_write_functions = self
            .supported_read_sub_functions()
            .into_iter()
            .filter(|s| SensorSubFunctionType::read_write_list().contains(s))
            .collect::<Vec<SensorSubFunctionType>>();

        for sub_type in supported_read_write_functions {
            states.insert(sub_type, self.read_raw(sub_type)?);
        }

        Ok(SensorState { states })
    }

    /// Writes the given state to this sensor.
    /// Returns an error and writes nothing if the given state contains one or more shared_subfunctions that this sensor does not support.
    fn write_state(&self, state: &SensorState) -> Result<()> {
        if let Some(&sub_type) = state
            .states
            .keys()
            .find(|s| !self.supported_write_sub_functions().contains(s))
        {
            return Err(Error::SubtypeNotSupported { sub_type });
        }

        self.write_state_lossy(state)
    }

    /// Writes the given state to this sensor.
    /// All subfunction types contained in the given state that are not supported by this sensor will be ignored.
    fn write_state_lossy(&self, state: &SensorState) -> Result<()> {
        for (&sub_type, raw_value) in &state.states {
            let path = self.subfunction_path(sub_type);
            if let Err(e) = write(&path, raw_value.as_bytes()) {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        return Err(Error::InsufficientRights { path })
                    }
                    std::io::ErrorKind::NotFound => continue,
                    _ => return Err(Error::Write { source: e, path }),
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
pub struct SensorState {
    states: HashMap<SensorSubFunctionType, String>,
}

#[cfg(feature = "writeable")]
impl SensorState {
    /// Returns a SensorState struct created from the given sensor.
    pub fn from_sensor(sensor: &impl WriteableSensor) -> Result<SensorState> {
        sensor.state()
    }

    /// Returns all subfunction types that this state contains.
    pub fn sub_types(&self) -> Vec<SensorSubFunctionType> {
        self.states.keys().cloned().collect()
    }
}

fn inspect_sensor<S: Sensor>(
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

#[cfg(test)]
mod tests;
