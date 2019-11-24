/*
 * Copyright (C) 2019  Malte Veerman <malte.veerman@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 */

//! A library that lets you use lm_sensor's sysfs interface from rust.
//!
//! Similar to libsensors this library lets you use the various sensors in your system.
//!
//! #Examples
//!
//! Print the temperature of all the temp sensors in your system:
//!
//! ```
//! use libmedium::parse_hwmons_read_only;
//! use libmedium::sensors::{Sensor, SensorBase};
//!
//! fn main() {
//!     let hwmons = parse_hwmons_read_only().unwrap();
//!     for (hwmon_index, hwmon_name, hwmon) in &hwmons {
//!         println!("hwmon{} with name {}:", hwmon_index, hwmon_name);
//!         for (_, temp_sensor) in hwmon.temps() {
//!             let temperature = temp_sensor.read_input().unwrap();
//!             println!("\t{}: {}", temp_sensor.name(), temperature);
//!         }
//!     }
//! }
//! ```
//!
//! Set the pwm value of all your pwm capable fans to full speed:
//!
//! ```
//! use libmedium::parse_hwmons_read_write;
//! use libmedium::sensors::pwm::{Pwm, PwmEnable, PwmSensor};
//!
//! fn main() {
//!     let hwmons = parse_hwmons_read_write().unwrap();
//!     for (_, _, hwmon) in &hwmons {
//!         for (_, pwm) in hwmon.pwms() {
//!             pwm.write_enable(PwmEnable::ManualControl).unwrap();
//!             pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
//!         }
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

use hwmon::*;

use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::path::{Path, PathBuf};

use snafu::Snafu;

const HWMON_PATH: &str = "/sys/class/hwmon/";

type ParsingResult<T> = std::result::Result<T, ParsingError>;

#[allow(missing_docs)]
#[derive(Debug, Snafu)]
pub enum ParsingError {
    /// You have insufficient rights. Try using the read only version of the parse_hwmons* functions.
    #[snafu(display("Insufficient rights for path {}", path.display()))]
    InsufficientRights { path: PathBuf },

    /// The path you are trying to parse is not valid.
    #[snafu(display("Invalid path to parse: {}", path.display()))]
    InvalidPath { path: PathBuf },

    /// The path you are trying to parse does not exist.
    #[snafu(display("Path does not exist: {}", path.display()))]
    PathDoesNotExist { path: PathBuf },

    /// Error which is returned if reading the name file of an hwmon fails.
    #[snafu(display("Error reading name file: {}", source))]
    NameFile { source: std::io::Error },

    /// Error when creating a new sensor.
    #[snafu(display("Error creating sensor of type {} with index {}", sensor_type, index))]
    SensorCreationError {
        sensor_type: &'static str,
        index: u16,
    },
}

/// This crate's central struct.
/// It stores all parsed hwmons which you can query either by name or by index.
/// You should not create this struct yourself but use the parse_hwmons* functions.
#[derive(Debug, Clone)]
pub struct Hwmons<H: Hwmon> {
    path: PathBuf,
    hwmons: HashMap<String, H>,
    names: BTreeMap<u16, String>,
}

impl<H: Hwmon> Hwmons<H> {
    /// The path that was parsed to generate this object.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get a hwmon by its name.
    /// Returns None if there is no hwmon with the given name.
    pub fn get_hwmon_by_name<T>(&self, name: &T) -> Option<&H>
    where
        T: Hash + Eq + ?Sized,
        String: Borrow<T>,
    {
        self.hwmons.get(name)
    }

    /// Get a hwmon by its index.
    /// Returns None if there is no hwmon with the given index.
    pub fn get_hwmon_by_index<T>(&self, index: &T) -> Option<&H>
    where
        T: Ord + ?Sized,
        u16: Borrow<T>,
    {
        if let Some(name) = self.names.get(&index) {
            self.hwmons.get(name)
        } else {
            None
        }
    }

    /// Returns an iterator over all hwmons, their names and their indexes.
    pub fn iter(&self) -> Iter<'_, H> {
        Iter {
            index: 0,
            hwmons: &self.hwmons,
            names: &self.names,
        }
    }
}

/// An iterator over all parsed hwmons.
#[derive(Debug, Copy, Clone)]
pub struct Iter<'a, H: Hwmon> {
    hwmons: &'a HashMap<String, H>,
    names: &'a BTreeMap<u16, String>,
    index: u16,
}

impl<'a, H: Hwmon> Iterator for Iter<'a, H> {
    type Item = (u16, &'a str, &'a H);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(name) = self.names.get(&self.index) {
            if let Some(hwmon) = self.hwmons.get(name) {
                self.index += 1;
                return Some((self.index - 1, name, hwmon));
            }
        }
        None
    }
}

impl<'a, H: Hwmon> IntoIterator for &'a Hwmons<H> {
    type Item = (u16, &'a str, &'a H);
    type IntoIter = Iter<'a, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn parse<H>(path: impl AsRef<Path>) -> ParsingResult<Hwmons<H>>
where
    H: Hwmon + Parseable<Parent = Hwmons<H>>,
{
    let path = path.as_ref();

    if !path.is_dir() {
        return Err(ParsingError::InvalidPath {
            path: path.to_path_buf(),
        });
    }

    let mut hwmons = Hwmons {
        path: path.to_path_buf(),
        names: BTreeMap::new(),
        hwmons: HashMap::new(),
    };

    for index in 0.. {
        match H::parse(&hwmons, index) {
            Ok(hwmon) => {
                let hwmon_name = hwmon.name().to_string();
                hwmons.names.insert(index, hwmon_name.clone());
                hwmons.hwmons.insert(hwmon_name, hwmon);
            }
            Err(e) => match e {
                ParsingError::PathDoesNotExist { .. } => break,
                e => return Err(e),
            },
        }
    }

    Ok(hwmons)
}

/// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
/// Be sure you have sufficient rights to write to your sensors. Usually only root has those rights.
#[cfg(feature = "writable")]
pub fn parse_hwmons_read_write() -> ParsingResult<Hwmons<ReadWriteHwmon>> {
    parse(HWMON_PATH)
}

/// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
pub fn parse_hwmons_read_only() -> ParsingResult<Hwmons<ReadOnlyHwmon>> {
    parse(HWMON_PATH)
}

/// Parses the given path and returns the found hwmons as a Hwmons object.
/// This function should only be used for debug and test purposes. Usually you should use
/// parse_hwmons_read_write() or parse_hwmons_read_only().
#[cfg(feature = "unrestricted_parsing")]
pub fn parse_path(path: impl AsRef<Path>) -> ParsingResult<Hwmons<ReadWriteHwmon>> {
    parse(path)
}

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> std::result::Result<Self, ParsingError>;
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

        let hwmons: Hwmons<ReadOnlyHwmon> = parse(test_path).unwrap();
        let hwmon0 = hwmons.get_hwmon_by_name("system").unwrap();
        let hwmon1 = hwmons.get_hwmon_by_name("other").unwrap();

        assert_eq!(hwmon0.name(), hwmons.get_hwmon_by_index(&0).unwrap().name());
        assert_eq!(hwmon1.name(), hwmons.get_hwmon_by_index(&1).unwrap().name());

        assert_eq!(hwmons.get_hwmon_by_index(&2).is_none(), true);
        assert_eq!(hwmons.get_hwmon_by_name("alias").is_none(), true);

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
