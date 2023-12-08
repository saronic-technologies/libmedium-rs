pub use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    io::Error as IoError,
    path::PathBuf,
};

use crate::sensors::SensorSubFunctionType;
use crate::units::Error as UnitError;

pub(super) type Result<T> = std::result::Result<T, Error>;

/// Error which can be returned from interacting with sensors.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// Error reading from sensor.
    Read { source: IoError, path: PathBuf },

    /// Error writing to sensor.
    Write { source: IoError, path: PathBuf },

    /// A UnitError occurred.
    UnitError { source: UnitError },

    /// You have insufficient rights. Try using the read only variant of whatever returned this error.
    InsufficientRights { path: PathBuf },

    /// The subfunction you requested is not supported by this sensor.
    SubtypeNotSupported { sub_type: SensorSubFunctionType },

    /// The sensor you tried to read from is faulty.
    FaultySensor,

    /// The sensor you tried to read from or write to is disabled.
    DisabledSensor,
}

impl Error {
    pub(crate) fn read(source: IoError, path: impl Into<PathBuf>) -> Self {
        Self::Read {
            source,
            path: path.into(),
        }
    }

    pub(crate) fn write(source: IoError, path: impl Into<PathBuf>) -> Self {
        Self::Write {
            source,
            path: path.into(),
        }
    }

    pub(crate) fn insufficient_rights(path: impl Into<PathBuf>) -> Self {
        Self::InsufficientRights { path: path.into() }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Read { source, .. } => Some(source),
            Error::Write { source, .. } => Some(source),
            Error::UnitError { source } => Some(source),
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
            Error::Read { path, source } => write!(
                f,
                "Reading from sensor at {} failed: {}",
                path.display(),
                source
            ),
            Error::Write { path, source } => write!(
                f,
                "Writing to sensor at {} failed: {}",
                path.display(),
                source
            ),
            Error::UnitError { source } => write!(f, "Raw sensor error: {}", source),
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

impl From<UnitError> for Error {
    fn from(raw_error: UnitError) -> Error {
        Error::UnitError { source: raw_error }
    }
}
