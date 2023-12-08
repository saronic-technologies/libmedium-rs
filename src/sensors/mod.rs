//! Module containing the sensors and their functionality.

#[cfg(feature = "sync")]
pub mod sync_sensors;

#[cfg(feature = "async")]
pub mod async_sensors;

mod error;
mod subfunction_type;

pub use error::Error;
pub use subfunction_type::SensorSubFunctionType;
