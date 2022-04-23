use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    io::Error as IoError,
    num::ParseIntError,
    path::PathBuf,
};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// The hwmon does not expose an update interval.
    UpdateIntervalNotAvailable,

    /// Error reading or writing to sysfs.
    Io { source: IoError, path: PathBuf },

    /// Error parsing string to integer value.
    Parsing {
        source: ParseIntError,
        path: PathBuf,
    },

    /// You have insufficient rights.
    InsufficientRights { path: PathBuf },
}

impl Error {
    pub(crate) fn update_interval_not_available() -> Self {
        Self::UpdateIntervalNotAvailable
    }

    pub(crate) fn io(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Io { source, path }
    }

    pub(crate) fn parsing(source: ParseIntError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Parsing { source, path }
    }

    pub(crate) fn insufficient_rights(path: impl Into<PathBuf>) -> Self {
        Self::InsufficientRights { path: path.into() }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::UpdateIntervalNotAvailable => None,
            Error::Io { source, .. } => Some(source),
            Error::Parsing { source, .. } => Some(source),
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
            Error::Parsing { source, path } => {
                write!(
                    f,
                    "Error parsing update interval at {}: {}",
                    path.display(),
                    source
                )
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
