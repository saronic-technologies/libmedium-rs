//! Module containing the humidity sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::Humidity;
use crate::{Parseable, ParsingResult};

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
        ReadOnlyHumidity {
            hwmon_path: write_humidity.hwmon_path,
            index: write_humidity.index,
        }
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

    fn try_from(value: ReadOnlyHumidity) -> std::result::Result<Self, Self::Error> {
        let read_write = ReadWriteHumidity {
            hwmon_path: value.hwmon_path,
            index: value.index,
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
