//! Module containing the current sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Current;
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all current sensors.
pub trait CurrSensor: SensorBase {}

impl<S: CurrSensor> Sensor<Current> for S {}
impl<S: CurrSensor> Min<Current> for S {}
impl<S: CurrSensor> Max<Current> for S {}
impl<S: CurrSensor> LowCrit<Current> for S {}
impl<S: CurrSensor> Crit<Current> for S {}
impl<S: CurrSensor> Average<Current> for S {}
impl<S: CurrSensor> Lowest<Current> for S {}
impl<S: CurrSensor> Highest<Current> for S {}

/// Struct that represents a read only current sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyCurr {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadOnlyCurr {
    /// Try converting this sensor into a read-write version of itself.
    pub fn try_into_read_write(self) -> Result<ReadWriteCurr> {
        let read_write = ReadWriteCurr {
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

impl SensorBase for ReadOnlyCurr {
    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyCurr {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(curr)
    }
}

impl CurrSensor for ReadOnlyCurr {}

#[cfg(feature = "writable")]
impl From<ReadWriteCurr> for ReadOnlyCurr {
    fn from(write_curr: ReadWriteCurr) -> ReadOnlyCurr {
        write_curr.into_read_only()
    }
}

/// Struct that represents a read/write current sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteCurr {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteCurr {
    fn base(&self) -> &'static str {
        "curr"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl ReadWriteCurr {
    /// Converts this sensor into a read-only version of itself.
    fn into_read_only(self) -> ReadOnlyCurr {
        ReadOnlyCurr {
            hwmon_path: self.hwmon_path,
            index: self.index,
        }
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteCurr {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let curr = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(curr)
    }
}

#[cfg(feature = "writable")]
impl CurrSensor for ReadWriteCurr {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteCurr {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyCurr> for ReadWriteCurr {
    type Error = Error;

    fn try_from(read_only: ReadOnlyCurr) -> std::result::Result<Self, Self::Error> {
        read_only.try_into_read_write()
    }
}
