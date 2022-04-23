//! A library that lets you use lm_sensor's sysfs interface from rust.
//!
//! Similar to libsensors this library lets you use the various sensors in your system.
//!
//! #Examples
//!
//! Print the temperature of all the temp sensors in your system:
//!
//! ```
//! use libmedium::{
//!     parse_hwmons,
//!     sensors::{Input, Sensor},
//! };
//!
//! let hwmons = parse_hwmons().unwrap();
//! for (hwmon_index, hwmon_name, hwmon) in &hwmons {
//!     println!("hwmon{} with name {}:", hwmon_index, hwmon_name);
//!     for (_, temp_sensor) in hwmon.temps() {
//!         let temperature = temp_sensor.read_input().unwrap();
//!         println!("\t{}: {}", temp_sensor.name(), temperature);
//!     }
//! }
//! ```
//!
//! Set the pwm value of all your pwm capable fans to full speed:
//!
//! ```
//! use libmedium::{
//!     parse_hwmons,
//!     sensors::WriteablePwmSensor,
//!     units::{Pwm, PwmEnable},
//! };
//!
//! let hwmons = parse_hwmons().unwrap();
//! for (_, _, hwmon) in &hwmons {
//!     for (_, pwm) in hwmon.writeable_pwms() {
//!         pwm.write_enable(PwmEnable::ManualControl).unwrap();
//!         pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
//!     }
//! }
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts
)]
#![forbid(unsafe_code, unstable_features)]

pub mod hwmon;
pub mod sensors;
pub mod units;

mod parsing;

pub use parsing::Error as ParsingError;

/// Convenience function for [`hwmon::Hwmons::parse`](crate::hwmon::Hwmons::parse())
pub fn parse_hwmons() -> Result<hwmon::Hwmons, ParsingError> {
    hwmon::Hwmons::parse()
}

#[cfg(test)]
mod tests;
