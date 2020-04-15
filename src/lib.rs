//! A library that lets you use lm_sensor's sysfs interface from rust.
//!
//! Similar to libsensors this library lets you use the various sensors in your system.
//!
//! #Examples
//!
//! Print the temperature of all the temp sensors in your system:
//!
//! ```
//! use libmedium::{
//!     Hwmon, Hwmons,
//!     sensors::{Sensor, SensorBase},
//! };
//!
//! let hwmons = Hwmons::parse_read_only().unwrap();
//! for (hwmon_index, hwmon_name, hwmon) in &hwmons {
//!     println!("hwmon{} with name {}:", hwmon_index, hwmon_name);
//!     for (_, temp_sensor) in hwmon.temps() {
//!         let temperature = temp_sensor.read_input().unwrap();
//!         println!("\t{}: {}", temp_sensor.name(), temperature);
//!     }
//! }
//! ```
//!
//! Set the pwm value of all your pwm capable fans to full speed:
//!
//! ```
//! use libmedium::{
//!     Hwmon, Hwmons,
//!     sensors::PwmSensor,
//!     units::{Pwm, PwmEnable},
//! };
//!
//! let hwmons = Hwmons::parse_read_write().unwrap();
//! for (_, _, hwmon) in &hwmons {
//!     for (_, pwm) in hwmon.pwms() {
//!         pwm.write_enable(PwmEnable::ManualControl).unwrap();
//!         pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
//!     }
//! }
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features
)]

pub mod hwmon;
pub mod sensors;
pub mod units;

pub use hwmon::Hwmon;

use hwmon::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::path::{Path, PathBuf};

const HWMON_PATH: &str = "/sys/class/hwmon/";

type ParsingResult<T> = std::result::Result<T, ParsingError>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum ParsingError {
    /// You have insufficient rights. Try using the read only version of the parse_hwmons* functions.
    InsufficientRights { path: PathBuf },

    /// The path you are trying to parse is not valid.
    InvalidPath { path: PathBuf },

    /// The path you are trying to parse does not exist.
    PathDoesNotExist { path: PathBuf },

    /// Everything else
    Other { source: std::io::Error },
}

impl Error for ParsingError {
    fn cause(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParsingError::InsufficientRights { .. } => None,
            ParsingError::InvalidPath { .. } => None,
            ParsingError::PathDoesNotExist { .. } => None,
            ParsingError::Other { source } => Some(source),
        }
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InsufficientRights { path } => {
                write!(f, "Insufficient rights for path {}", path.display())
            }
            ParsingError::InvalidPath { path } => {
                write!(f, "Invalid path to parse: {}", path.display())
            }
            ParsingError::PathDoesNotExist { path } => {
                write!(f, "Path does not exist: {}", path.display())
            }
            ParsingError::Other { .. } => write!(f, "I/O error"),
        }
    }
}

/// This crate's central struct.
/// It stores all parsed hwmons which you can query either by name or by index.
#[derive(Debug, Clone)]
pub struct Hwmons<H: Hwmon> {
    path: PathBuf,
    hwmons: Vec<H>,
}

impl<H: Hwmon> Hwmons<H> {
    /// The path that was parsed to generate this object.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get `Hwmon`s by their name.
    /// Returns an empty iterator, if there is no `Hwmon` with the given name.
    pub fn hwmons_by_name(&self, name: impl AsRef<str>) -> impl Iterator<Item = &H> {
        self.hwmons
            .iter()
            .filter(move |hwmon| hwmon.name() == name.as_ref())
    }

    /// Get a `Hwmon` by its index.
    /// Returns None if there is no `Hwmon` with the given index.
    pub fn hwmon_by_index(&self, index: usize) -> Option<&H> {
        self.hwmons.get(index)
    }

    /// Returns an iterator over all hwmons, their names and their indices.
    pub fn iter(&self) -> Iter<'_, H> {
        Iter {
            index: 0,
            hwmons: &self.hwmons,
        }
    }

    fn parse(path: impl AsRef<Path>) -> ParsingResult<Self>
    where
        H: Parseable<Parent = Self>,
    {
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
            match H::parse(&hwmons, index) {
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

impl Hwmons<ReadOnlyHwmon> {
    /// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
    pub fn parse_read_only() -> ParsingResult<Self> {
        Self::parse(HWMON_PATH)
    }
}

#[cfg(feature = "writable")]
impl Hwmons<ReadWriteHwmon> {
    /// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
    /// Be sure you have sufficient rights to write to your sensors. Usually only root has those rights.
    pub fn parse_read_write() -> ParsingResult<Self> {
        Self::parse(HWMON_PATH)
    }

    /// Parses the given path and returns the found hwmons as a `Hwmons` object.
    /// This function should only be used for debug and test purposes. Usually you should use
    /// parse_read_write() or parse_read_only().
    #[cfg(feature = "unrestricted_parsing")]
    pub fn parse_path(path: impl AsRef<Path>) -> ParsingResult<Self> {
        Self::parse(path)
    }
}

/// An iterator over all parsed hwmons.
#[derive(Debug, Copy, Clone)]
pub struct Iter<'a, H: Hwmon> {
    hwmons: &'a [H],
    index: usize,
}

impl<'a, H: Hwmon> Iterator for Iter<'a, H> {
    type Item = (usize, &'a str, &'a H);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(hwmon) = self.hwmons.get(self.index) {
            self.index += 1;
            return Some((self.index - 1, hwmon.name(), hwmon));
        }
        None
    }
}

impl<'a, H: Hwmon> FusedIterator for Iter<'a, H> {}

impl<'a, H: Hwmon> ExactSizeIterator for Iter<'a, H> {
    fn len(&self) -> usize {
        self.hwmons.len() - self.index
    }
}

impl<'a, H: Hwmon> IntoIterator for &'a Hwmons<H> {
    type Item = (usize, &'a str, &'a H);
    type IntoIter = Iter<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::fs::{remove_dir_all, File, OpenOptions};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    pub struct VirtualHwmonBuilder {
        root: PathBuf,
        index: u16,
    }

    impl VirtualHwmonBuilder {
        pub fn create(
            root: impl AsRef<Path>,
            index: u16,
            name: impl AsRef<[u8]>,
        ) -> VirtualHwmonBuilder {
            let path = root.as_ref().join(format!("hwmon{}", index));

            fs::create_dir_all(&path).unwrap();

            File::create(path.join("name"))
                .unwrap()
                .write(name.as_ref())
                .unwrap();

            VirtualHwmonBuilder {
                root: root.as_ref().to_path_buf(),
                index,
            }
        }

        pub fn add_temp(
            self,
            index: u16,
            value: i32,
            label: impl AsRef<str>,
        ) -> VirtualHwmonBuilder {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(format!("temp{}_input", index)))
                .unwrap()
                .write(value.to_string().as_bytes())
                .unwrap();

            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(format!("temp{}_enable", index)))
                .unwrap()
                .write(b"1\n")
                .unwrap();

            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(format!("temp{}_label", index)))
                .unwrap()
                .write(label.as_ref().as_bytes())
                .unwrap();

            self
        }

        pub fn add_fan(self, index: u16, value: u32) -> VirtualHwmonBuilder {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(format!("fan{}_input", index)))
                .unwrap()
                .write(value.to_string().as_bytes())
                .unwrap();

            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(format!("fan{}_enable", index)))
                .unwrap()
                .write(b"1\n")
                .unwrap();

            self
        }

        pub fn add_pwm(
            self,
            index: u16,
            create_enable_file: bool,
            create_mode_file: bool,
        ) -> VirtualHwmonBuilder {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(&format!("pwm{}", index)))
                .unwrap()
                .write(b"0\n")
                .unwrap();
            if create_enable_file {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(self.path().join(&format!("pwm{}_enable", index)))
                    .unwrap()
                    .write(b"2\n")
                    .unwrap();
            }
            if create_mode_file {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(self.path().join(&format!("pwm{}_mode", index)))
                    .unwrap()
                    .write(b"1\n")
                    .unwrap();
            }

            self.add_fan(index, 1000)
        }

        pub fn path(&self) -> PathBuf {
            self.root.join(format!("hwmon{}", self.index))
        }
    }

    #[test]
    fn test_parse() {
        let test_path = Path::new("test_parse");

        VirtualHwmonBuilder::create(test_path, 0, "system")
            .add_pwm(1, true, true)
            .add_pwm(2, true, true)
            .add_temp(1, 40000, "temp1")
            .add_temp(2, 60000, "temp2");
        VirtualHwmonBuilder::create(test_path, 1, "other")
            .add_pwm(1, true, true)
            .add_temp(1, 40000, "temp1")
            .add_fan(2, 1000);

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon0 = hwmons.hwmons_by_name("system").next().unwrap();
        let hwmon1 = hwmons.hwmons_by_name("other").next().unwrap();

        assert_eq!(hwmon0.name(), hwmons.hwmon_by_index(0).unwrap().name());
        assert_eq!(hwmon1.name(), hwmons.hwmon_by_index(1).unwrap().name());

        assert_eq!(hwmons.hwmon_by_index(2).is_none(), true);
        assert_eq!(hwmons.hwmons_by_name("alias").next().is_none(), true);

        assert_eq!(hwmon0.temps().len(), 2);
        assert_eq!(hwmon1.temps().len(), 1);
        assert_eq!(hwmon0.pwms().len(), 2);
        assert_eq!(hwmon1.pwms().len(), 1);

        hwmon0.pwms().get(&1u16).unwrap();
        hwmon0.pwms().get(&2u16).unwrap();
        hwmon1.pwms().get(&1u16).unwrap();
        hwmon0.temps().get(&1u16).unwrap();
        hwmon0.temps().get(&2u16).unwrap();
        hwmon1.temps().get(&1u16).unwrap();

        remove_dir_all(test_path).unwrap();
    }
}
