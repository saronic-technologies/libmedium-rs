//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Humidity;
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all humidity sensors.
pub trait HumiditySensor: SensorBase {}

impl<S: HumiditySensor> Sensor<Humidity> for S {}

/// Struct that represents a read only humidity sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyHumidity {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadOnlyHumidity {
    /// Try converting this sensor into a read-write version of itself.
    pub fn try_into_read_write(self) -> Result<ReadWriteHumidity> {
        let read_write = ReadWriteHumidity {
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

impl SensorBase for ReadOnlyHumidity {
    fn base(&self) -> &'static str {
        "humidity"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyHumidity {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity)
    }
}

impl HumiditySensor for ReadOnlyHumidity {}

#[cfg(feature = "writable")]
impl From<ReadWriteHumidity> for ReadOnlyHumidity {
    fn from(write_humidity: ReadWriteHumidity) -> ReadOnlyHumidity {
        write_humidity.into_read_only()
    }
}

/// Struct that represents a read/write humidity sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteHumidity {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadWriteHumidity {
    /// Converts this sensor into a read-only version of itself.
    pub fn into_read_only(self) -> ReadOnlyHumidity {
        ReadOnlyHumidity {
            hwmon_path: self.hwmon_path,
            index: self.index,
        }
    }
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteHumidity {
    fn base(&self) -> &'static str {
        "humidity"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteHumidity {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let humidity = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(humidity)
    }
}

#[cfg(feature = "writable")]
impl HumiditySensor for ReadWriteHumidity {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteHumidity {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyHumidity> for ReadWriteHumidity {
    type Error = Error;

    fn try_from(read_only: ReadOnlyHumidity) -> std::result::Result<Self, Self::Error> {
        read_only.try_into_read_write()
    }
}
