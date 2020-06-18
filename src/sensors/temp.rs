//! Module containing the temp sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::{Raw, TempType, Temperature};
use crate::{Parseable, ParsingResult};

#[cfg(feature = "writable")]
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

/// Trait implemented by all temp sensors.
pub trait TempSensor: SensorBase {
    /// Reads the type subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_type(&self) -> Result<TempType> {
        let raw = self.read_raw(SensorSubFunctionType::Type)?;
        TempType::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the offset subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_offset(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Offset)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the max_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_max_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MaxHyst)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the min_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_min_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::MinHyst)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the crit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_crit_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::CritHyst)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the emergency subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_emergency(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::Emergency)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the emergency_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_emergency_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::EmergencyHyst)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the lcrit_hyst subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn read_lcrit_hyst(&self) -> Result<Temperature> {
        let raw = self.read_raw(SensorSubFunctionType::LowCritHyst)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }

    /// Converts offset and writes it to this temp's offset subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_offset(&self, offset: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Offset, &offset.to_raw())
    }

    /// Converts max_hyst and writes it to this temp's max_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_max_hyst(&self, max_hyst: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::MaxHyst, &max_hyst.to_raw())
    }

    /// Converts min_hyst and writes it to this temp's min_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_min_hyst(&self, min_hyst: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::MinHyst, &min_hyst.to_raw())
    }

    /// Converts crit_hyst and writes it to this temp's crit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_crit_hyst(&self, crit_hyst: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::CritHyst, &crit_hyst.to_raw())
    }

    /// Converts emergency and writes it to this temp's emergency subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_emergency(&self, emergency: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::Emergency, &emergency.to_raw())
    }

    /// Converts emergency_hyst and writes it to this temp's emergency_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_emergency_hyst(&self, emergency_hyst: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(
            SensorSubFunctionType::EmergencyHyst,
            &emergency_hyst.to_raw(),
        )
    }

    /// Converts lcrit_hyst and writes it to this temp's lcrit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    #[cfg(feature = "writable")]
    fn write_lcrit_hyst(&self, lcrit_hyst: Temperature) -> Result<()>
    where
        Self: WritableSensorBase,
    {
        self.write_raw(SensorSubFunctionType::LowCritHyst, &lcrit_hyst.to_raw())
    }
}

impl<S: TempSensor + Faulty> Sensor<Temperature> for S {
    /// Reads the input subfunction of this temp sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> Result<Temperature> {
        if self.read_faulty().unwrap_or(false) {
            return Err(Error::FaultySensor);
        }

        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Temperature::from_raw(&raw).map_err(Error::from)
    }
}

impl<S: TempSensor> Min<Temperature> for S {}
impl<S: TempSensor> Max<Temperature> for S {}
impl<S: TempSensor> Crit<Temperature> for S {}
impl<S: TempSensor> LowCrit<Temperature> for S {}

/// Struct that represents a read only temp sensor.
#[derive(Debug, Clone)]
pub struct ReadOnlyTemp {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for ReadOnlyTemp {
    fn base(&self) -> &'static str {
        "temp"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

impl Parseable for ReadOnlyTemp {
    type Parent = ReadOnlyHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(temp)
    }
}

impl TempSensor for ReadOnlyTemp {}
impl Faulty for ReadOnlyTemp {}

#[cfg(feature = "writable")]
impl From<ReadWriteTemp> for ReadOnlyTemp {
    fn from(write_temp: ReadWriteTemp) -> ReadOnlyTemp {
        ReadOnlyTemp {
            hwmon_path: write_temp.hwmon_path,
            index: write_temp.index,
        }
    }
}

/// Struct that represents a read/write temp sensor.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteTemp {
    hwmon_path: PathBuf,
    index: u16,
}

#[cfg(feature = "writable")]
impl SensorBase for ReadWriteTemp {
    fn base(&self) -> &'static str {
        "temp"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteTemp {
    type Parent = ReadWriteHwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(temp)
    }
}

#[cfg(feature = "writable")]
impl TempSensor for ReadWriteTemp {}
#[cfg(feature = "writable")]
impl Faulty for ReadWriteTemp {}
#[cfg(feature = "writable")]
impl WritableSensorBase for ReadWriteTemp {}

#[cfg(feature = "writable")]
impl TryFrom<ReadOnlyTemp> for ReadWriteTemp {
    type Error = Error;

    fn try_from(value: ReadOnlyTemp) -> std::result::Result<Self, Self::Error> {
        let read_write = ReadWriteTemp {
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
