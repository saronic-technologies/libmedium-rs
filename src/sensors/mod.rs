/*
 * Copyright (C) 2019  Malte Veerman <malte.veerman@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

//! Module containing the sensors and their functionality.

mod curr;
mod energy;
mod fan;
mod humidity;
mod power;
mod pwm;
mod subfunction;
mod temp;
mod voltage;

pub use curr::*;
pub use energy::*;
pub use fan::*;
pub use humidity::*;
pub use power::*;
pub use pwm::*;
pub use subfunction::*;
pub use temp::*;
pub use voltage::*;

use crate::hwmon::*;
use crate::units::{Raw, RawError};
use crate::{ParsingError, ParsingResult};

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::Duration;

type SensorResult<T> = std::result::Result<T, SensorError>;

/// Error which can be returned from interacting with sensors.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum SensorError {
    /// Error reading from sensor.
    Read {
        source: std::io::Error,
        path: PathBuf,
    },

    /// Error writing to sensor.
    Write {
        source: std::io::Error,
        path: PathBuf,
    },

    /// A RawSensorError occurred.
    RawSensorError { source: RawError },

    /// You have insufficient rights. Try using the read only variant of whatever returned this error.
    InsufficientRights { path: PathBuf },

    /// The subfunction you requested ist not supported by this sensor.
    SubtypeNotSupported { sub_type: SensorSubFunctionType },

    /// The sensor you tried to read from is faulty.
    FaultySensor,

    /// The sensor you tried to read from or write to is disabled.
    DisabledSensor,
}

impl Error for SensorError {
    fn cause(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SensorError::Read { source, .. } => Some(source),
            SensorError::Write { source, .. } => Some(source),
            SensorError::RawSensorError { source } => Some(source),
            SensorError::InsufficientRights { .. } => None,
            SensorError::SubtypeNotSupported { .. } => None,
            SensorError::FaultySensor => None,
            SensorError::DisabledSensor => None,
        }
    }
}

impl Display for SensorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::Read { path, .. } => {
                write!(f, "Reading from sensor at {} failed", path.display())
            }
            SensorError::Write { path, .. } => {
                write!(f, "Writing to sensor at {} failed", path.display())
            }
            SensorError::RawSensorError { .. } => write!(f, "Raw sensor error"),
            SensorError::InsufficientRights { path } => write!(
                f,
                "You have insufficient rights to read/write {}",
                path.display()
            ),
            SensorError::SubtypeNotSupported { sub_type } => {
                write!(f, "Sensor does not support the subtype {}", sub_type)
            }
            SensorError::FaultySensor => write!(f, "The sensor is faulty"),
            SensorError::DisabledSensor => write!(f, "The sensor is disabled"),
        }
    }
}

impl From<RawError> for SensorError {
    fn from(raw_error: RawError) -> SensorError {
        SensorError::RawSensorError { source: raw_error }
    }
}

/// Base trait that all sensors must implement.
/// It contains the functionality to get a sensor's name, index or supported subfunctions.
pub trait SensorBase {
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
    fn read_raw(&self, sub_type: SensorSubFunctionType) -> SensorResult<String> {
        let path = self.subfunction_path(sub_type);

        match read_to_string(&path) {
            Ok(s) => Ok(s.trim().to_string()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(SensorError::SubtypeNotSupported { sub_type }),
                std::io::ErrorKind::PermissionDenied => {
                    Err(SensorError::InsufficientRights { path })
                }
                _ => Err(SensorError::Read { source: e, path }),
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

/// Base trait that all writable sensors must implement.
#[cfg(feature = "writable")]
pub trait WritableSensorBase: SensorBase {
    /// Returns a list of all writable subfunction types supported by this sensor.
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
    fn write_raw(&self, sub_type: SensorSubFunctionType, raw_value: &str) -> SensorResult<()> {
        let path = self.subfunction_path(sub_type);

        write(&path, raw_value.as_bytes()).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => SensorError::SubtypeNotSupported { sub_type },
            std::io::ErrorKind::PermissionDenied => SensorError::InsufficientRights { path },
            _ => SensorError::Read { source: e, path },
        })
    }

    /// Resets this sensor's history.
    /// Returns an error if this functionality is not supported by the sensor.
    fn reset_history(&self) -> SensorResult<()> {
        self.write_raw(SensorSubFunctionType::ResetHistory, &true.to_raw())
    }

    /// Returns a SensorState struct that represents the state of all writable subfunctions of this sensor.
    fn state(&self) -> SensorResult<SensorState> {
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
    /// Returns an error and writes nothing if the given state contains one or more subfunctions that this sensor does not support.
    fn write_state(&self, state: &SensorState) -> SensorResult<()> {
        if let Some(&sub_type) = state
            .states
            .keys()
            .find(|s| !self.supported_write_sub_functions().contains(s))
        {
            return Err(SensorError::SubtypeNotSupported { sub_type });
        }

        self.write_state_lossy(state)
    }

    /// Writes the given state to this sensor.
    /// All subfunction types contained in the given state that are not supported by this sensor will be ignored.
    fn write_state_lossy(&self, state: &SensorState) -> SensorResult<()> {
        for (&sub_type, raw_value) in &state.states {
            let path = self.subfunction_path(sub_type);
            if let Err(e) = write(&path, raw_value.as_bytes()) {
                match e.kind() {
                    std::io::ErrorKind::PermissionDenied => {
                        return Err(SensorError::InsufficientRights { path })
                    }
                    std::io::ErrorKind::NotFound => continue,
                    _ => return Err(SensorError::Write { source: e, path }),
                }
            }
        }

        Ok(())
    }
}

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to use the input and enable subfunctions.
pub trait Sensor<P: Raw>: SensorBase {
    /// Reads the input subfunction of this sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }

    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_enable(&self) -> SensorResult<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        bool::from_raw(&raw).map_err(SensorError::from)
    }

    /// Enables or disables this sensor.
    /// Returns an error if this functionality is not supported by the sensor.
    #[cfg(feature = "writable")]
    fn write_enable(&self, enable: bool) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }
}

/// Trait implemented by all sensors that can have a faulty subfunction.
pub trait Faulty: SensorBase {
    /// Reads whether this sensor is faulty or not.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_faulty(&self) -> SensorResult<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Fault)?;
        bool::from_raw(&raw).map_err(SensorError::from)
    }
}

/// Trait implemented by all sensors that can have a min subfunction.
pub trait Min<P: Raw>: SensorBase {
    /// Reads this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_min(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Min)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }

    /// Writes this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    #[cfg(feature = "writable")]
    fn write_min(&self, min: P) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Min, &min.to_raw())
    }
}

/// Trait implemented by all sensors that can have a max subfunction.
pub trait Max<P: Raw>: SensorBase {
    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_max(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Max)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }

    /// Writes this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    #[cfg(feature = "writable")]
    fn write_max(&self, max: P) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw())
    }
}

/// Trait implemented by all sensors that can have an average subfunction.
pub trait Average<P: Raw>: SensorBase {
    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_average(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Average)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }
}

/// Trait implemented by all sensors that can have a lowest subfunction.
pub trait Lowest<P: Raw>: SensorBase {
    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lowest(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }
}

/// Trait implemented by all sensors that can have a highest subfunction.
pub trait Highest<P: Raw>: SensorBase {
    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_highest(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Highest)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }
}

/// Trait implemented by all sensors that can have a crit subfunction.
pub trait Crit<P: Raw>: SensorBase {
    /// Reads this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_crit(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::Crit)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }

    /// Writes this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    #[cfg(feature = "writable")]
    fn write_crit(&self, crit: P) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw())
    }
}

/// Trait implemented by all sensors that can have a lcrit subfunction.
pub trait LowCrit<P: Raw>: SensorBase {
    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lcrit(&self) -> SensorResult<P> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit)?;
        P::from_raw(&raw).map_err(SensorError::from)
    }

    /// Writes this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    #[cfg(feature = "writable")]
    fn write_lcrit(&self, lcrit: P) -> SensorResult<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
    }
}

/// A struct that represents the state of all writable subfunctions of a sensor.
/// It can be used to reset a sensor to a previous state or copy its settings to another sensor.
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "writable")]
pub struct SensorState {
    states: HashMap<SensorSubFunctionType, String>,
}

#[cfg(feature = "writable")]
impl SensorState {
    /// Returns a SensorState struct created from the given sensor.
    pub fn from_sensor(sensor: &impl WritableSensorBase) -> SensorResult<SensorState> {
        sensor.state()
    }

    /// Returns all subfunction types that this state contains.
    pub fn sub_types(&self) -> Vec<SensorSubFunctionType> {
        self.states.keys().cloned().collect()
    }
}

fn inspect_sensor<Sensor: SensorBase>(sensor: Sensor) -> ParsingResult<Sensor> {
    let mut count = 0;

    for sub in SensorSubFunctionType::read_list() {
        let path = sensor.subfunction_path(sub);

        if path.exists() {
            if path.metadata().is_err() {
                return Err(ParsingError::InsufficientRights { path });
            }

            count += 1;
        }
    }

    if count == 0 {
        Err(ParsingError::InvalidPath {
            path: sensor
                .hwmon_path()
                .join(format!("{}{}", sensor.base(), sensor.index(),)),
        })
    } else {
        Ok(sensor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensors::{ReadOnlyFan, ReadOnlyTemp};
    use crate::tests::*;
    use crate::units::Temperature;
    use crate::{Hwmons, Parseable};

    use std::fs::remove_dir_all;
    use std::path::Path;

    #[test]
    fn test_sensor_read_value() {
        let test_path = Path::new("test_sensor_read_value");

        VirtualHwmonBuilder::create(test_path, 0, "system")
            .add_temp(1, 40000, "temp1")
            .add_fan(1, 60);

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let temp = ReadOnlyTemp::parse(hwmon, 1).unwrap();
        let fan = ReadOnlyFan::parse(hwmon, 1).unwrap();

        #[cfg(not(feature = "measurements_units"))]
        assert_eq!(
            Temperature::from_degrees_celsius(40.0),
            temp.read_input().unwrap()
        );

        #[cfg(feature = "measurements_units")]
        assert_eq!(Temperature::from_celsius(40.0), temp.read_input().unwrap());

        #[cfg(not(feature = "measurements_units"))]
        assert_eq!(60, fan.read_input().unwrap().as_rpm());

        #[cfg(feature = "measurements_units")]
        assert_eq!(60.0, fan.read_input().unwrap().as_hertz() * 60.0);

        remove_dir_all(test_path).unwrap();
    }

    #[test]
    fn test_label() {
        let test_path = Path::new("test_label");

        VirtualHwmonBuilder::create(test_path, 0, "system").add_temp(1, 40000, "test_temp1\n");

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let temp = ReadOnlyTemp::parse(hwmon, 1).unwrap();

        assert_eq!(temp.name(), String::from("test_temp1"));

        remove_dir_all(test_path).unwrap();
    }
}
