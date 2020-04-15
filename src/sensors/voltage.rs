//! Module containing the voltage sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Voltage;
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all voltage sensors.
pub trait VoltSensor: SensorBase {}

impl<S: VoltSensor> Sensor<Voltage> for S {}
impl<S: VoltSensor> Min<Voltage> for S {}
impl<S: VoltSensor> Max<Voltage> for S {}
impl<S: VoltSensor> LowCrit<Voltage> for S {}
impl<S: VoltSensor> Crit<Voltage> for S {}
impl<S: VoltSensor> Average<Voltage> for S {}
impl<S: VoltSensor> Lowest<Voltage> for S {}
impl<S: VoltSensor> Highest<Voltage> for S {}

/// Struct that represents a read only voltage sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyVolt {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyVolt {
    fn base(&self) -> &'static str {
        "in"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyVolt {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(volt)
    }
}

impl VoltSensor for ReadOnlyVolt {}

#[cfg(feature = "writable")]
impl From<ReadWriteVolt> for ReadOnlyVolt {
    fn from(write_voltage: ReadWriteVolt) -> ReadOnlyVolt {
        ReadOnlyVolt {
            hwmon_path: write_voltage.hwmon_path,
            index: write_voltage.index,
        }
    }
}

/// Struct that represents a read/write voltage sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteVolt {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteVolt {
    fn base(&self) -> &'static str {
        "in"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteVolt {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let volt = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(volt)
    }
}

#[cfg(feature = "writable")]
impl VoltSensor for ReadWriteVolt {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteVolt {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyVolt> for ReadWriteVolt {
    type Error = SensorError;

    fn try_from(value: ReadOnlyVolt) -> Result<Self, Self::Error> {
        let read_write = ReadWriteVolt {
            hwmon_path: value.hwmon_path,
            index: value.index,
        };

        if read_write.supported_write_sub_functions().is_empty() {
            return Err(SensorError::InsufficientRights {
                path: read_write.hwmon_path.join(format!(
                    "{}{}",
                    read_write.base(),
                    read_write.index(),
                )),
            });
        }

        Ok(read_write)
    }
}
