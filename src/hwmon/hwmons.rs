use super::Hwmon;
use crate::parsing::{Error as ParsingError, Parseable, Result as ParsingResult};

use std::iter::FusedIterator;
use std::path::{Path, PathBuf};

const HWMON_PATH: &str = "/sys/class/hwmon/";

/// This crate's central struct.
/// It stores all parsed [`Hwmon`](crate::hwmon::Hwmon)s which you can query either by name, device path or index.
#[derive(Debug, Clone)]
pub struct Hwmons {
    path: PathBuf,
    hwmons: Vec<Hwmon>,
}

impl Hwmons {
    /// The path that was parsed to generate this object.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get `Hwmon`s by their name.
    /// Returns an empty iterator, if there is no `Hwmon` with the given name.
    pub fn hwmons_by_name(&self, name: impl AsRef<str>) -> impl Iterator<Item = &Hwmon> {
        self.hwmons
            .iter()
            .filter(move |hwmon| hwmon.name() == name.as_ref())
    }

    /// Get a `Hwmon` by its index.
    /// Returns `None`, if there is no `Hwmon` with the given index.
    pub fn hwmon_by_index(&self, index: usize) -> Option<&Hwmon> {
        self.hwmons.get(index)
    }

    /// Get a `Hwmon` by its device path.
    /// Returns `None`, if there is no `Hwmon` with the given device path.
    pub fn hwmon_by_device_path(&self, device_path: impl AsRef<Path>) -> Option<&Hwmon> {
        self.hwmons
            .iter()
            .find(move |&hwmon| hwmon.device_path() == device_path.as_ref())
    }

    /// Returns an iterator over all hwmons, their names and their indices.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            index: 0,
            hwmons: &self.hwmons,
        }
    }

    /// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
    pub fn parse() -> ParsingResult<Self> {
        Self::parse_path(HWMON_PATH)
    }

    /// Parses the provided path and returns the found hwmons as a Hwmons object.
    #[cfg(feature = "unrestricted_parsing")]
    pub fn parse_unrestricted(path: impl AsRef<Path>) -> ParsingResult<Self> {
        Self::parse_path(path)
    }

    pub(crate) fn parse_path(path: impl AsRef<Path>) -> ParsingResult<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ParsingError::PathDoesNotExist {
                path: path.to_path_buf(),
            });
        }

        if !path.is_dir() {
            return Err(ParsingError::InvalidPath {
                path: path.to_path_buf(),
            });
        }

        let mut hwmons = Hwmons {
            path: path.to_path_buf(),
            hwmons: Vec::new(),
        };

        for index in 0.. {
            match Hwmon::parse(&hwmons, index) {
                Ok(hwmon) => {
                    hwmons.hwmons.push(hwmon);
                }
                Err(e) => match e {
                    ParsingError::PathDoesNotExist { .. } => break,
                    e => return Err(e),
                },
            }
        }

        Ok(hwmons)
    }
}

/// An iterator over all parsed hwmons.
#[derive(Debug, Copy, Clone)]
pub struct Iter<'a> {
    hwmons: &'a [Hwmon],
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, &'a str, &'a Hwmon);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hwmon) = self.hwmons.get(self.index) {
            self.index += 1;
            return Some((self.index - 1, hwmon.name(), hwmon));
        }
        None
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        self.hwmons.len() - self.index
    }
}

impl<'a> IntoIterator for &'a Hwmons {
    type Item = (usize, &'a str, &'a Hwmon);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
