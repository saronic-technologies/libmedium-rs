//! A library that lets you use lm_sensor's sysfs interface from rust.
//!
//! Similar to libsensors this library lets you use the various sensors in your system.
//!
//! #Examples
//!
//! Print the temperature of all the temp sensors in your system:
//!
//! ```no_run
//! use libmedium::{
//!     parse_hwmons,
//!     sensors::sync_sensors::{Sensor, temp::TempSensor},
//! };
//!
//! let hwmons = parse_hwmons().unwrap();
//! for hwmon in &hwmons {
//!     println!("hwmon{} with name {}:", hwmon.index(), hwmon.name());
//!     for (_, temp_sensor) in hwmon.temps() {
//!         let temperature = temp_sensor.read_input().unwrap();
//!         println!("\t{}: {:?}", temp_sensor.name(), temperature);
//!     }
//! }
//! ```
//!
//! Set the pwm value of all your pwm capable fans to full speed:
//!
//! ```no_run
//! use libmedium::{
//!     parse_hwmons,
//!     sensors::sync_sensors::pwm::WriteablePwmSensor,
//!     units::{Pwm, PwmEnable},
//! };
//!
//! let hwmons = parse_hwmons().unwrap();
//! for hwmon in &hwmons {
//!     for (_, pwm) in hwmon.writeable_pwms() {
//!         pwm.write_enable(PwmEnable::ManualControl).unwrap();
//!         pwm.write_pwm(Pwm::try_from_percent(100.0).unwrap()).unwrap();
//!     }
//! }
//! ```

pub mod hwmon;
pub mod sensors;
pub mod units;

mod parsing;

pub use parsing::Error as ParsingError;

/// Convenience function for [`hwmon::sync_hwmon::Hwmons::parse`](crate::hwmon::sync_hwmon::Hwmons::parse())
#[cfg(feature = "sync")]
pub fn parse_hwmons() -> Result<hwmon::sync_hwmon::Hwmons, ParsingError> {
    hwmon::sync_hwmon::Hwmons::parse()
}

/// Convenience function for [`hwmon::async_hwmon::Hwmons::parse`](crate::hwmon::async_hwmon::Hwmons::parse())
#[cfg(feature = "async")]
pub async fn parse_hwmons_async() -> Result<hwmon::async_hwmon::Hwmons, ParsingError> {
    hwmon::async_hwmon::Hwmons::parse().await
}

#[cfg(test)]
mod tests;
