use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    io::Error as IoError,
    path::PathBuf,
};

pub(super) type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// Error listing hwmons
    Hwmons { source: IoError, path: PathBuf },

    /// Error reading hwmon name file
    HwmonName { source: IoError, path: PathBuf },

    /// Error listing the contents of the hwmon directory
    HwmonDir { source: IoError, path: PathBuf },

    /// Error parsing sensor
    Sensor { source: IoError, path: PathBuf },
}

impl Error {
    pub(crate) fn hwmons(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Hwmons { source, path }
    }

    pub(crate) fn hwmon_name(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::HwmonName { source, path }
    }

    pub(crate) fn hwmon_dir(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::HwmonDir { source, path }
    }

    pub(crate) fn sensor(source: IoError, path: impl Into<PathBuf>) -> Self {
        let path = path.into();

        Error::Sensor { source, path }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Hwmons { source, .. } => Some(source),
            Error::HwmonName { source, .. } => Some(source),
            Error::HwmonDir { source, .. } => Some(source),
            Error::Sensor { source, .. } => Some(source),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Hwmons { source, path } => {
                write!(f, "Error listing hwmons at {}: {}", path.display(), source)
            }
            Error::HwmonName { source, path } => write!(
                f,
                "Error reading hwmon name file at {}: {}",
                path.display(),
                source
            ),
            Error::HwmonDir { source, path } => write!(
                f,
                "Error listing contents of hwmon directory at {}: {}",
                path.display(),
                source
            ),
            Error::Sensor { source, path } => {
                write!(f, "Error parsing sensor at {}: {}", path.display(), source)
            }
        }
    }
}

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> Result<Self>;
}
