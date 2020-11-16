//! Module containing the energy sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Energy;
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
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

#[cfg(feature = "writable")]
impl ReadOnlyEnergy {
    /// Try converting this sensor into a read-write version of itself.
    pub fn try_into_read_write(self) -> Result<ReadWriteEnergy> {
        let read_write = ReadWriteEnergy {
            hwmon_path: self.hwmon_path,
            index: self.index,
        };

        if read_write.supported_write_sub_functions().is_empty() {
            return Err(Error::InsufficientRights {
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
        write_energy.into_read_only()
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
impl ReadWriteEnergy {
    /// Converts this sensor into a read-only version of itself.
    fn into_read_only(self) -> ReadOnlyEnergy {
        ReadOnlyEnergy {
            hwmon_path: self.hwmon_path,
            index: self.index,
        }
    }
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
    type Error = Error;

    fn try_from(read_only: ReadOnlyEnergy) -> std::result::Result<Self, Self::Error> {
        read_only.try_into_read_write()
    }
}
