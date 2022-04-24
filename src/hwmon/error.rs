use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    io::Error as IoError,
    path::PathBuf,
};

use crate::units::Error as UnitError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// The hwmon does not expose an update interval.
    UpdateIntervalNotAvailable,

    /// The hwmon does not expose the beep_enable functionality.
    BeepEnable,

    /// Error reading or writing to sysfs.
    Io { source: IoError, path: PathBuf },

    /// Unit conversion error.
    Unit { source: UnitError, path: PathBuf },

    /// You have insufficient rights.
    InsufficientRights { path: PathBuf },
}

impl Error {
    pub(crate) fn update_interval_not_available() -> Self {
        Self::UpdateIntervalNotAvailable
    }

    pub(crate) fn beep_enable() -> Self {
        Self::BeepEnable
    }

    pub(crate) fn io(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Io { source, path }
    }

    pub(crate) fn unit(source: UnitError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Unit { source, path }
    }

    pub(crate) fn insufficient_rights(path: impl Into<PathBuf>) -> Self {
        Self::InsufficientRights { path: path.into() }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::UpdateIntervalNotAvailable => None,
            Error::BeepEnable => None,
            Error::Io { source, .. } => Some(source),
            Error::Unit { source, .. } => Some(source),
            Error::InsufficientRights { .. } => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UpdateIntervalNotAvailable => {
                write!(f, "Hwmon does not expose an update interval")
            }
            Error::BeepEnable => {
                write!(f, "Hwmon does not expose the beep_enable functionality")
            }
            Error::Unit { source, path } => {
                write!(f, "Unit conversion error at {}: {}", path.display(), source)
            }
            Error::Io { source, path } => {
                write!(f, "Io error at {}: {}", path.display(), source)
            }
            Error::InsufficientRights { path } => {
                write!(
                    f,
                    "You have insufficient rights to write to {}",
                    path.display()
                )
            }
        }
    }
}
