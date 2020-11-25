use std::{
    error::Error as StdError,
    fmt::{Display, Formatter},
    path::PathBuf,
};

pub(super) type Result<T> = std::result::Result<T, Error>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Error {
    /// You have insufficient rights.
    InsufficientRights { path: PathBuf },

    /// The path you are trying to parse is not valid.
    InvalidPath { path: PathBuf },

    /// The path you are trying to parse does not exist.
    PathDoesNotExist { path: PathBuf },

    /// Everything else
    Other { source: std::io::Error },
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::InsufficientRights { .. } => None,
            Error::InvalidPath { .. } => None,
            Error::PathDoesNotExist { .. } => None,
            Error::Other { source } => Some(source),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InsufficientRights { path } => {
                write!(f, "Insufficient rights for path {}", path.display())
            }
            Error::InvalidPath { path } => write!(f, "Invalid path to parse: {}", path.display()),
            Error::PathDoesNotExist { path } => {
                write!(f, "Path does not exist: {}", path.display())
            }
            Error::Other { .. } => write!(f, "I/O error"),
        }
    }
}

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> Result<Self>;
}
