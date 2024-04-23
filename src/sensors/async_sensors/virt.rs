//! Module containing the virtual sensors and their related functionality.

use super::*;
use crate::units::Raw;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use tokio::fs::read_to_string;

#[async_trait]
/// Helper trait that sums up all functionality of a read-only virtual sensor.
pub trait AsyncVirtualSensor<T: Raw>: std::fmt::Debug {
    /// Returns the path to this virtual sensor's underlying file.
    fn path(&self) -> &Path;

    /// Reads the virtual sensor.
    async fn read(&self) -> Result<T> {
        match read_to_string(self.path()).await {
            Ok(s) => Ok(T::from_raw(s.trim())?),
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => Err(Error::InsufficientRights {
                    path: self.path().to_path_buf(),
                }),
                _ => Err(Error::read(e, self.path())),
            },
        }
    }
}

/// Struct that represents a read only virtual sensor.
#[derive(Debug, Clone)]
pub(crate) struct VirtualSensorStruct {
    path: PathBuf,
}

impl<T: Raw> AsyncVirtualSensor<T> for VirtualSensorStruct {
    fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(feature = "writeable")]
#[async_trait]
/// Helper trait that sums up all functionality of a read-write virtual sensor.
pub trait AsyncWriteableVirtualSensor<T: Raw + Sync>: AsyncVirtualSensor<T> {
    /// Writes to the virtual sensor.
    async fn write(&self, value: &T) -> Result<()> {
        tokio::fs::write(self.path(), value.to_raw().as_bytes())
            .await
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::PermissionDenied => Error::insufficient_rights(self.path()),
                _ => Error::write(e, self.path()),
            })
    }
}

#[cfg(feature = "writeable")]
impl<T: Raw + Sync> AsyncWriteableVirtualSensor<T> for VirtualSensorStruct {}

/// Creates a virtual sensor from the given file at `path`.
pub fn virtual_sensor_from_path<T: Raw>(
    path: impl Into<PathBuf>,
) -> Result<impl AsyncVirtualSensor<T> + Clone + Send + Sync> {
    let path = path.into();

    if !path.is_file() {
        return Err(Error::read(
            std::io::Error::from(std::io::ErrorKind::NotFound),
            path,
        ));
    }

    Ok(VirtualSensorStruct { path })
}

#[cfg(feature = "writeable")]
/// Creates a virtual sensor from the given file at `path`.
pub fn writeable_virtual_sensor_from_path<T: Raw + Sync>(
    path: impl Into<PathBuf>,
) -> Result<impl AsyncWriteableVirtualSensor<T> + Clone + Send + Sync> {
    let path = path.into();

    if !path.is_file() {
        return Err(Error::Read {
            path,
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        });
    }

    Ok(VirtualSensorStruct { path })
}
