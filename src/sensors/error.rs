pub use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    path::PathBuf,
};

use crate::units::RawError;

use super::SensorSubFunctionType;

pub(super) type Result<T> = std::result::Result<T, Error>;

/// Error which can be returned from interacting with sensors.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// Error reading from sensor.
    Read {
        source: std::io::Error,
        path: PathBuf,
    },

    /// Error writing to sensor.
    Write {
        source: std::io::Error,
        path: PathBuf,
    },

    /// A RawError occurred.
    RawError { source: RawError },

    /// You have insufficient rights. Try using the read only variant of whatever returned this error.
    InsufficientRights { path: PathBuf },

    /// The subfunction you requested ist not supported by this sensor.
    SubtypeNotSupported { sub_type: SensorSubFunctionType },

    /// The sensor you tried to read from is faulty.
    FaultySensor,

    /// The sensor you tried to read from or write to is disabled.
    DisabledSensor,
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Read { source, .. } => Some(source),
            Error::Write { source, .. } => Some(source),
            Error::RawError { source } => Some(source),
            Error::InsufficientRights { .. } => None,
            Error::SubtypeNotSupported { .. } => None,
            Error::FaultySensor => None,
            Error::DisabledSensor => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Read { path, .. } => {
                write!(f, "Reading from sensor at {} failed", path.display())
            }
            Error::Write { path, .. } => {
                write!(f, "Writing to sensor at {} failed", path.display())
            }
            Error::RawError { .. } => write!(f, "Raw sensor error"),
            Error::InsufficientRights { path } => write!(
                f,
                "You have insufficient rights to read/write {}",
                path.display()
            ),
            Error::SubtypeNotSupported { sub_type } => {
                write!(f, "Sensor does not support the subtype {}", sub_type)
            }
            Error::FaultySensor => write!(f, "The sensor is faulty"),
            Error::DisabledSensor => write!(f, "The sensor is disabled"),
        }
    }
}

impl From<RawError> for Error {
    fn from(raw_error: RawError) -> Error {
        Error::RawError { source: raw_error }
    }
}
