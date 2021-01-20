//! Module containing the temp sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::parsing::{Parseable, Result as ParsingResult};
use crate::units::{Raw, TempType, Temperature};

use std::path::{Path, PathBuf};

/// Helper trait that sums up all functionality of a read-only temp sensor.
pub trait TempSensor:
    Sensor<Value = Temperature>
    + Enable
    + Input
    + Min
    + Max
    + Crit
    + LowCrit
    + Faulty
    + std::fmt::Debug
    + Clone
{
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
}

/// Struct that represents a read only temp sensor.
#[derive(Debug, Clone)]
pub(crate) struct TempSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl Sensor for TempSensorStruct {
    type Value = Temperature;

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

impl Parseable for TempSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let temp = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(temp)
    }
}

impl Input for TempSensorStruct {
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

impl Enable for TempSensorStruct {}
impl Min for TempSensorStruct {}
impl Max for TempSensorStruct {}
impl Crit for TempSensorStruct {}
impl LowCrit for TempSensorStruct {}
impl TempSensor for TempSensorStruct {}
impl Faulty for TempSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensor for TempSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write temp sensor.
pub trait WriteableTempSensor:
    TempSensor
    + WriteableSensor
    + WriteableEnable
    + WriteableMin
    + WriteableMax
    + WriteableCrit
    + WriteableLowCrit
{
    /// Converts offset and writes it to this temp's offset subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_offset(&self, offset: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Offset, &offset.to_raw())
    }

    /// Converts max_hyst and writes it to this temp's max_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_max_hyst(&self, max_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::MaxHyst, &max_hyst.to_raw())
    }

    /// Converts min_hyst and writes it to this temp's min_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_min_hyst(&self, min_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::MinHyst, &min_hyst.to_raw())
    }

    /// Converts crit_hyst and writes it to this temp's crit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_crit_hyst(&self, crit_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::CritHyst, &crit_hyst.to_raw())
    }

    /// Converts emergency and writes it to this temp's emergency subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_emergency(&self, emergency: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Emergency, &emergency.to_raw())
    }

    /// Converts emergency_hyst and writes it to this temp's emergency_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_emergency_hyst(&self, emergency_hyst: Temperature) -> Result<()> {
        self.write_raw(
            SensorSubFunctionType::EmergencyHyst,
            &emergency_hyst.to_raw(),
        )
    }

    /// Converts lcrit_hyst and writes it to this temp's lcrit_hyst subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_lcrit_hyst(&self, lcrit_hyst: Temperature) -> Result<()> {
        self.write_raw(SensorSubFunctionType::LowCritHyst, &lcrit_hyst.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteableTempSensor for TempSensorStruct {}
