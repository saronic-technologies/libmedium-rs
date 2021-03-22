//! Module containing the sensors and their functionality.

mod curr;
mod energy;
mod error;
mod fan;
mod humidity;
mod power;
mod pwm;
mod subfunction;
mod temp;
mod virt;
mod voltage;

pub use curr::*;
pub use energy::*;
pub use error::Error;
pub use fan::*;
pub use humidity::*;
pub use power::*;
pub use pwm::*;
pub use subfunction::*;
pub use temp::*;
pub use virt::*;
pub use voltage::*;

use crate::hwmon::*;
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

    /// Returns a SensorState struct that represents the state of all writeable subfunctions of this sensor.
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
    /// Returns an error and writes nothing if the given state contains one or more subfunctions that this sensor does not support.
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

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to read the input subfunctions.
pub trait Input: Sensor {
    /// Reads the input subfunction of this sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to read the enable subfunctions.
pub trait Enable: Sensor {
    /// Reads whether or not this sensor is enabled.
    /// Returns an error, if the sensor doesn't support the feature.
    fn read_enable(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Enable)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors except for pwm.
#[cfg(feature = "writeable")]
pub trait WriteableEnable: WriteableSensor {
    /// Sets this sensor's enabled state.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_enable(&self, enable: bool) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Enable> WriteableEnable for S {}

/// Trait implemented by all sensors that can have a faulty subfunction.
pub trait Faulty: Sensor {
    /// Reads whether this sensor is faulty or not.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_faulty(&self) -> Result<bool> {
        let raw = self.read_raw(SensorSubFunctionType::Fault)?;
        bool::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that can have a min subfunction.
pub trait Min: Sensor {
    /// Reads this sensor's min value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_min(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Min)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a min subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableMin: WriteableSensor {
    /// Writes this sensor's min value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_min(&self, min: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Min, &min.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Min> WriteableMin for S {}

/// Trait implemented by all sensors that can have a max subfunction.
pub trait Max: Sensor {
    /// Reads this sensor's max value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_max(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Max)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a max subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableMax: WriteableSensor {
    /// Writes this sensor's max value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_max(&self, max: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Max, &max.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Max> WriteableMax for S {}

/// Trait implemented by all sensors that can have an average subfunction.
pub trait Average: Sensor {
    /// Reads this sensor's average value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_average(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Average)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that can have a lowest subfunction.
pub trait Lowest: Sensor {
    /// Reads this sensor's historically lowest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lowest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Lowest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that can have a highest subfunction.
pub trait Highest: Sensor {
    /// Reads this sensor's historically highest input.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_highest(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Highest)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all sensors that can have a crit subfunction.
pub trait Crit: Sensor {
    /// Reads this sensor's crit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_crit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Crit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a crit subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableCrit: WriteableSensor {
    /// Writes this sensor's crit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_crit(&self, crit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Crit, &crit.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + Crit> WriteableCrit for S {}

/// Trait implemented by all sensors that can have a lcrit subfunction.
pub trait LowCrit: Sensor {
    /// Reads this sensor's lcrit value.
    /// Returns an error, if this sensor doesn't support the feature.
    fn read_lcrit(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::LowCrit)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}

/// Trait implemented by all writeable sensors that can have a lcrit subfunction.
#[cfg(feature = "writeable")]
pub trait WriteableLowCrit: WriteableSensor {
    /// Writes this sensor's lcrit value.
    /// Returns an error, if the sensor doesn't support the feature.
    fn write_lcrit(&self, lcrit: Self::Value) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCrit, &lcrit.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl<S: WriteableSensor + LowCrit> WriteableLowCrit for S {}

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

fn inspect_sensor<S: Input>(sensor: S) -> ParsingResult<S> {
    if let Err(e) = sensor
        .subfunction_path(SensorSubFunctionType::Input)
        .metadata()
    {
        return Err(ParsingError::sensor(
            e,
            sensor.subfunction_path(SensorSubFunctionType::Input),
        ));
    }

    Ok(sensor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hwmon::Hwmons;
    use crate::parsing::Parseable;
    use crate::sensors::TempSensorStruct;
    use crate::tests::*;

    use temp_dir::TempDir;

    #[test]
    fn test_sensor_read_value() {
        let test_dir = TempDir::new().unwrap();

        VirtualHwmonBuilder::create(test_dir.path(), 0, "system")
            .add_temp(1, 40000, "temp1")
            .add_fan(1, 60);

        let hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let temp = TempSensorStruct::parse(hwmon, 1).unwrap();
        let fan = FanSensorStruct::parse(hwmon, 1).unwrap();

        #[cfg(not(feature = "uom_units"))]
        assert_eq!(40.0, temp.read_input().unwrap().as_degrees_celsius());

        #[cfg(feature = "uom_units")]
        assert_eq!(
            40.0,
            temp.read_input()
                .unwrap()
                .round::<uom::si::thermodynamic_temperature::degree_celsius>()
                .get::<uom::si::thermodynamic_temperature::degree_celsius>()
        );

        #[cfg(not(feature = "uom_units"))]
        assert_eq!(60, fan.read_input().unwrap().as_rpm());

        #[cfg(feature = "uom_units")]
        assert_eq!(
            60.0,
            fan.read_input()
                .unwrap()
                .round::<uom::si::angular_velocity::revolution_per_minute>()
                .get::<uom::si::angular_velocity::revolution_per_minute>()
        );
    }

    #[test]
    fn test_label() {
        let test_dir = TempDir::new().unwrap();

        VirtualHwmonBuilder::create(test_dir.path(), 0, "system").add_temp(
            1,
            40000,
            "test_temp1\n",
        );

        let hwmons: Hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let temp = TempSensorStruct::parse(hwmon, 1).unwrap();

        assert_eq!(temp.name(), String::from("test_temp1"));
    }
}
