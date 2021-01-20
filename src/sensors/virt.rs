//! Module containing the virtual sensors and their related functionality.

use super::*;
use crate::units::Raw;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

/// Helper trait that sums up all functionality of a read-only virtual sensor.
pub trait VirtualSensor<T: Raw>: std::fmt::Debug + Clone {
    /// Returns the path to this virtual sensor's underlying file.
    fn path(&self) -> &Path;

    /// Reads the virtual sensor.
    fn read(&self) -> Result<T> {
        match read_to_string(self.path()) {
            Ok(s) => Ok(T::from_raw(s.trim())?),
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => Err(Error::InsufficientRights {
                    path: self.path().to_path_buf(),
                }),
                _ => Err(Error::Read {
                    source: e,
                    path: self.path().to_path_buf(),
                }),
            },
        }
    }
}

/// Struct that represents a read only virtual sensor.
#[derive(Debug, Clone)]
pub(crate) struct VirtualSensorStruct {
    path: PathBuf,
}

impl<T: Raw> VirtualSensor<T> for VirtualSensorStruct {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(feature = "writeable")]
/// Helper trait that sums up all functionality of a read-write virtual sensor.
pub trait WriteableVirtualSensor<T: Raw>: VirtualSensor<T> {
    /// Writes to the virtual sensor.
    fn write(&self, value: &T) -> Result<()> {
        std::fs::write(&self.path(), value.to_raw().as_bytes()).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => Error::InsufficientRights {
                path: self.path().to_path_buf(),
            },
            _ => Error::Write {
                source: e,
                path: self.path().to_path_buf(),
            },
        })
    }
}

#[cfg(feature = "writeable")]
impl<T: Raw> WriteableVirtualSensor<T> for VirtualSensorStruct {}

/// Creates a virtual sensor from the given file at `path`.
pub fn virtual_sensor_from_path<T: Raw>(
    path: impl Into<PathBuf>,
) -> Result<impl VirtualSensor<T> + Clone + Send + Sync> {
    let path = path.into();

    if !path.is_file() {
        return Err(Error::Read {
            path,
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        });
    }

    Ok(VirtualSensorStruct { path })
}

#[cfg(feature = "writeable")]
/// Creates a virtual sensor from the given file at `path`.
pub fn writeable_virtual_sensor_from_path<T: Raw>(
    path: impl Into<PathBuf>,
) -> Result<impl WriteableVirtualSensor<T> + Clone + Send + Sync> {
    let path = path.into();

    if !path.is_file() {
        return Err(Error::Read {
            path,
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        });
    }

    Ok(VirtualSensorStruct { path })
}
