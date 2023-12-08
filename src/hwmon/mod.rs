//! Module containing the Hwmon struct and related functionality.

mod error;

#[cfg(feature = "sync")]
pub mod sync_hwmon;

#[cfg(feature = "async")]
pub mod async_hwmon;

pub use error::Error;
