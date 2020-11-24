//! Module containing the power sensors and their related functionality.

use super::*;
use crate::hwmon::*;
use crate::units::{Power, Ratio, Raw};
use crate::{Parseable, ParsingResult};

/// Helper trait that sums up all the functionality of a read-only power sensor.
pub trait PowerSensor:
    Enable
    + Sensor<Power>
    + Max<Power>
    + Crit<Power>
    + Average<Power>
    + Highest<Power>
    + Lowest<Power>
    + std::fmt::Debug
{
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
}

/// Struct that represents a read only power sensor.
#[derive(Debug, Clone)]
pub(crate) struct PowerSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl SensorBase for PowerSensorStruct {
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

impl Parseable for PowerSensorStruct {
    type Parent = Hwmon;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let power = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(power)
    }
}

impl Enable for PowerSensorStruct {}
impl Sensor<Power> for PowerSensorStruct {}
impl Max<Power> for PowerSensorStruct {}
impl Crit<Power> for PowerSensorStruct {}
impl Average<Power> for PowerSensorStruct {}
impl Highest<Power> for PowerSensorStruct {}
impl Lowest<Power> for PowerSensorStruct {}
impl PowerSensor for PowerSensorStruct {}

#[cfg(feature = "writeable")]
impl WriteableSensorBase for PowerSensorStruct {}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all the functionality of a read-write power sensor.
pub trait WriteablePowerSensor:
    PowerSensor + WriteableSensorBase + WriteableEnable + WriteableMax<Power> + WriteableCrit<Power>
{
    /// Converts cap and writes it to the cap subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_cap(&self, cap: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Cap, &cap.to_raw())
    }

    /// Converts cap_hyst and writes it to the cap_hyst subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_cap_hyst(&self, cap_hyst: Power) -> Result<()> {
        self.write_raw(SensorSubFunctionType::CapHyst, &cap_hyst.to_raw())
    }

    /// Converts interval and writes it to the average_interval subfunction of this power sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    fn write_average_interval(&self, interval: Duration) -> Result<()> {
        self.write_raw(SensorSubFunctionType::AverageInterval, &interval.to_raw())
    }
}

#[cfg(feature = "writeable")]
impl WriteablePowerSensor for PowerSensorStruct {}
