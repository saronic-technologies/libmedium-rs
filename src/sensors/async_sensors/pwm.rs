//! Module containing the pwm sensors and their related functionality.

use super::*;
use crate::parsing::{AsyncParseable, Result as ParsingResult};
use crate::units::{Frequency, Pwm, PwmEnable, PwmMode, Raw};

use std::path::Path;

#[async_trait]
/// Helper trait that sums up all functionality of a read-only pwm sensor.
pub trait AsyncPwmSensor: AsyncSensor<Value = Pwm> + std::fmt::Debug {
    /// Reads the pwm subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_pwm(&self) -> Result<Pwm> {
        let raw = self.read_raw(SensorSubFunctionType::Pwm).await?;
        Pwm::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the enable subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_enable(&self) -> Result<PwmEnable> {
        let raw = self.read_raw(SensorSubFunctionType::Enable).await?;
        PwmEnable::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the mode subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_mode(&self) -> Result<PwmMode> {
        let raw = self.read_raw(SensorSubFunctionType::Mode).await?;
        PwmMode::from_raw(&raw).map_err(Error::from)
    }

    /// Reads the freq subfunction of this pwm sensor.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn read_frequency(&self) -> Result<Frequency> {
        let raw = self.read_raw(SensorSubFunctionType::Freq).await?;
        Frequency::from_raw(&raw).map_err(Error::from)
    }
}

/// Struct that represents a read only pwm sensor.
#[derive(Debug, Clone)]
pub(crate) struct PwmSensorStruct {
    hwmon_path: PathBuf,
    index: u16,
}

impl AsyncSensor for PwmSensorStruct {
    type Value = Pwm;

    fn base(&self) -> &'static str {
        "pwm"
    }

    fn index(&self) -> u16 {
        self.index
    }

    fn hwmon_path(&self) -> &Path {
        self.hwmon_path.as_path()
    }
}

#[async_trait]
impl AsyncParseable for PwmSensorStruct {
    type Parent = Hwmon;

    async fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let pwm = Self {
            hwmon_path: parent.path().to_path_buf(),
            index,
        };

        inspect_sensor(pwm, SensorSubFunctionType::Pwm).await
    }

    fn prefix() -> &'static str {
        "pwm"
    }
}

impl AsyncPwmSensor for PwmSensorStruct {}

#[cfg(feature = "writeable")]
impl AsyncWriteableSensor for PwmSensorStruct {}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write pwm sensor.
pub trait AsyncWriteablePwmSensor: AsyncPwmSensor + AsyncWriteableSensor {
    /// Converts pwm and writes it to this pwm's pwm subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_pwm(&self, pwm: Pwm) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Pwm, &pwm.to_raw()).await
    }

    /// Converts enable and writes it to this pwm's enable subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_enable(&self, enable: PwmEnable) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Enable, &enable.to_raw()).await
    }

    /// Converts mode and writes it to this pwm's mode subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_mode(&self, mode: PwmMode) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Mode, &mode.to_raw()).await
    }

    /// Converts freq and writes it to this pwm's freq subfunction.
    /// Returns an error, if this sensor doesn't support the subfunction.
    async fn write_frequency(&self, freq: Frequency) -> Result<()> {
        self.write_raw(SensorSubFunctionType::Freq, &freq.to_raw()).await
    }
}

#[cfg(feature = "writeable")]
impl AsyncWriteablePwmSensor for PwmSensorStruct {}
