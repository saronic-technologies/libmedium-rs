//! Module containing the energy sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Energy;
use crate::{Parseable, ParsingResult};

use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all energy sensors.
pub trait EnergySensor: SensorBase {}

impl<S: EnergySensor> Sensor<Energy> for S {}

/// Struct that represents a read only energy sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyEnergy {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyEnergy {
    fn base(&self) -> &'static str {
        "energy"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyEnergy {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(energy)
    }
}

impl EnergySensor for ReadOnlyEnergy {}

#[cfg(feature = "writable")]
impl From<ReadWriteEnergy> for ReadOnlyEnergy {
    fn from(write_energy: ReadWriteEnergy) -> ReadOnlyEnergy {
        ReadOnlyEnergy {
            hwmon_path: write_energy.hwmon_path,
            index: write_energy.index,
        }
    }
}

/// Struct that represents a read/write energy sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteEnergy {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteEnergy {
    fn base(&self) -> &'static str {
        "energy"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteEnergy {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let energy = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(energy)
    }
}

#[cfg(feature = "writable")]
impl EnergySensor for ReadWriteEnergy {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteEnergy {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyEnergy> for ReadWriteEnergy {
    type Error = SensorError;

    fn try_from(value: ReadOnlyEnergy) -> Result<Self, Self::Error> {
        let read_write = ReadWriteEnergy {
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
