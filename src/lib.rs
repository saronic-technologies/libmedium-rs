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
//!     Hwmons,
//!     sensors::{Sensor, SensorBase},
//! };
//!
//! let hwmons = Hwmons::parse().unwrap();
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
//!     Hwmons,
//!     sensors::WriteablePwmSensor,
//!     units::{Pwm, PwmEnable},
//! };
//!
//! let hwmons = Hwmons::parse().unwrap();
//! for (_, _, hwmon) in &hwmons {
//!     for (_, pwm) in hwmon.writeable_pwms() {
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
    trivial_numeric_casts
)]
#![forbid(unsafe_code, unstable_features)]

pub mod hwmon;
pub mod sensors;
pub mod units;

mod parsing;

pub use hwmon::Hwmon;
pub use parsing::Error as ParsingError;

pub(crate) use parsing::{Parseable, Result as ParsingResult};

use std::iter::FusedIterator;
use std::path::{Path, PathBuf};

const HWMON_PATH: &str = "/sys/class/hwmon/";

/// This crate's central struct.
/// It stores all parsed hwmons which you can query either by name or by index.
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

    fn parse_path(path: impl AsRef<Path>) -> ParsingResult<Self> {
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

        let hwmons = Hwmons::parse_path(test_path).unwrap();
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
