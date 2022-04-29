use crate::{
    sensors::{Error, Result, Sensor, SensorSubFunctionType},
    units::Raw,
};

/// Trait implemented by all sensors except for pwm.
/// It contains the functionality to read the input subfunction.
pub trait Input: Sensor {
    /// Reads the input subfunction of this sensor.
    /// Returns an error, if this sensor doesn't support the subtype.
    fn read_input(&self) -> Result<Self::Value> {
        let raw = self.read_raw(SensorSubFunctionType::Input)?;
        Self::Value::from_raw(&raw).map_err(Error::from)
    }
}
