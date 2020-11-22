//! Module containing the power sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::{Power, Ratio, Raw};
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
use std::convert::TryFrom;

/// Trait implemented by all power sensors.
pub trait PowerSensor: SensorBase {
    /// Reads the accuracy subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_accuracy(&self) -> Result<Ratio> {
        let raw = self.read_raw(SensorSubFunctionType::Accuracy)?;
        Ratio::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::Cap)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMax)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapMin)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_cap_hyst(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::CapHyst)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageInterval)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_max(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMax)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_interval_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_interval_min(&self) -> Result<Duration> {
        let raw = self.read_raw(SensorSubFunctionType::AverageIntervalMin)?;
        Duration::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_highest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_highest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageHighest)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_lowest subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_lowest(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageLowest)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_max subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_max(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMax)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the average_min subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_average_min(&self) -> Result<Power> {
        let raw = self.read_raw(SensorSubFunctionType::AverageMin)?;
        Power::from_raw(&raw).map_err(Error::from)
    }

    /// Converts cap and writes it to the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_cap(&self, cap: Power) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Cap, &cap.to_raw())
    }

    /// Converts cap_hyst and writes it to the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_cap_hyst(&self, cap_hyst: Power) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::CapHyst, &cap_hyst.to_raw())
    }

    /// Converts interval and writes it to the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_average_interval(&self, interval: Duration) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::AverageInterval, &interval.to_raw())
    }
}

impl<S: PowerSensor> Sensor<Power> for S {}
impl<S: PowerSensor> Max<Power> for S {}
impl<S: PowerSensor> Crit<Power> for S {}
impl<S: PowerSensor> Average<Power> for S {}
impl<S: PowerSensor> Highest<Power> for S {}
impl<S: PowerSensor> Lowest<Power> for S {}

/// Struct that represents a read only power sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyPower {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadOnlyPower {
    /// Try converting this sensor into a read-write version of itself.
    pub fn try_into_read_write(self) -> Result<ReadWritePower> {
        let read_write = ReadWritePower {
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

impl SensorBase for ReadOnlyPower {
    fn base(&self) -> &'static str {
        "power"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyPower {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(power)
    }
}

impl PowerSensor for ReadOnlyPower {}

#[cfg(feature = "writable")]
impl From<ReadWritePower> for ReadOnlyPower {
    fn from(write_power: ReadWritePower) -> ReadOnlyPower {
        write_power.into_read_only()
    }
}

/// Struct that represents a read/write power sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWritePower {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl ReadWritePower {
    /// Converts this sensor into a read-only version of itself.
    pub fn into_read_only(self) -> ReadOnlyPower {
        ReadOnlyPower {
            hwmon_path: self.hwmon_path,
            index: self.index,
        }
    }
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWritePower {
    fn base(&self) -> &'static str {
        "power"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWritePower {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(power)
    }
}

#[cfg(feature = "writable")]
impl PowerSensor for ReadWritePower {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWritePower {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyPower> for ReadWritePower {
    type Error = Error;

    fn try_from(read_only: ReadOnlyPower) -> std::result::Result<Self, Self::Error> {
        read_only.try_into_read_write()
    }
}
