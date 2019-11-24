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

//! Module containing the Hwmon struct and related functionality.

use super::{Hwmons, Parseable, ParsingError, ParsingResult};
use crate::sensors::curr::*;
use crate::sensors::energy::*;
use crate::sensors::fan::*;
use crate::sensors::humidity::*;
use crate::sensors::power::*;
use crate::sensors::pwm::*;
use crate::sensors::temp::*;
use crate::sensors::voltage::*;
use crate::sensors::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

fn check_path(path: impl AsRef<Path>) -> ParsingResult<()> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ParsingError::PathDoesNotExist {
            path: path.to_path_buf(),
        });
    }

    if !path.is_dir() {
        return Err(ParsingError::InvalidPath { path: path.into() });
    }

    if let Err(e) = path.metadata() {
        match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                return Err(ParsingError::InsufficientRights { path: path.into() })
            }
            _ => return Err(ParsingError::InvalidPath { path: path.into() }),
        }
    }

    Ok(())
}

fn get_name(path: impl AsRef<Path>) -> ParsingResult<String> {
    let path = path.as_ref();

    let name_path = path.join("name");
    let name = match read_to_string(&name_path) {
        Ok(name) => name.trim().to_string(),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                return Err(ParsingError::InvalidPath { path: name_path })
            }
            std::io::ErrorKind::PermissionDenied => {
                return Err(ParsingError::InsufficientRights { path: name_path })
            }
            _ => return Err(ParsingError::NameFile { source: e }),
        },
    };

    Ok(name)
}

fn init_sensors<S, H>(hwmon: &H, start_index: u16) -> ParsingResult<BTreeMap<u16, S>>
where
    S: SensorBase + Parseable<Parent = H> + Clone + Any,
    H: Hwmon,
{
    let mut sensors = BTreeMap::new();
    for index in start_index.. {
        match S::parse(hwmon, index) {
            Ok(sensor) => {
                sensors.insert(index, sensor);
            }
            Err(sensor_error) => match sensor_error {
                ParsingError::InsufficientRights { path } => {
                    return Err(ParsingError::InsufficientRights { path })
                }
                _ => break,
            },
        }
    }

    Ok(sensors)
}

/// Base trait that all hwmon must implement.
pub trait Hwmon {
    /// Returns the hwmon's name.
    fn name(&self) -> &str;

    /// Returns the hwmon's path.
    fn path(&self) -> &Path;
}

/// The read only variant of Hwmon. It contains all sensors found whithin its directory path.
#[derive(Debug, Clone)]
pub struct ReadOnlyHwmon {
    name: String,
    path: PathBuf,
    currs: BTreeMap<u16, ReadOnlyCurr>,
    energies: BTreeMap<u16, ReadOnlyEnergy>,
    fans: BTreeMap<u16, ReadOnlyFan>,
    humidities: BTreeMap<u16, ReadOnlyHumidity>,
    powers: BTreeMap<u16, ReadOnlyPower>,
    pwms: BTreeMap<u16, ReadOnlyPwm>,
    temps: BTreeMap<u16, ReadOnlyTemp>,
    voltages: BTreeMap<u16, ReadOnlyVolt>,
}

impl ReadOnlyHwmon {
    /// Returns all current sensors found in this hwmon.
    pub fn currents(&self) -> &BTreeMap<u16, ReadOnlyCurr> {
        &self.currs
    }

    /// Returns all energy sensors found in this hwmon.
    pub fn energies(&self) -> &BTreeMap<u16, ReadOnlyEnergy> {
        &self.energies
    }

    /// Returns all fan sensors found in this hwmon.
    pub fn fans(&self) -> &BTreeMap<u16, ReadOnlyFan> {
        &self.fans
    }

    /// Returns all humidity sensors found in this hwmon.
    pub fn humidities(&self) -> &BTreeMap<u16, ReadOnlyHumidity> {
        &self.humidities
    }

    /// Returns all power sensors found in this hwmon.
    pub fn powers(&self) -> &BTreeMap<u16, ReadOnlyPower> {
        &self.powers
    }

    /// Returns all pwm sensors found in this hwmon.
    pub fn pwms(&self) -> &BTreeMap<u16, ReadOnlyPwm> {
        &self.pwms
    }

    /// Returns all temp sensors found in this hwmon.
    pub fn temps(&self) -> &BTreeMap<u16, ReadOnlyTemp> {
        &self.temps
    }

    /// Returns all voltage sensors found in this hwmon.
    pub fn voltages(&self) -> &BTreeMap<u16, ReadOnlyVolt> {
        &self.voltages
    }
}

impl Hwmon for ReadOnlyHwmon {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &Path {
        &self.path.as_path()
    }
}

impl Parseable for ReadOnlyHwmon {
    type Parent = Hwmons<Self>;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let path = parent.path().join(format!("hwmon{}", index));

        check_path(&path)?;

        let mut hwmon = Self {
            name: get_name(&path)?,
            path,
            currs: BTreeMap::new(),
            energies: BTreeMap::new(),
            fans: BTreeMap::new(),
            humidities: BTreeMap::new(),
            powers: BTreeMap::new(),
            pwms: BTreeMap::new(),
            temps: BTreeMap::new(),
            voltages: BTreeMap::new(),
        };

        hwmon.currs = init_sensors(&hwmon, 1)?;
        hwmon.energies = init_sensors(&hwmon, 1)?;
        hwmon.fans = init_sensors(&hwmon, 1)?;
        hwmon.humidities = init_sensors(&hwmon, 1)?;
        hwmon.powers = init_sensors(&hwmon, 1)?;
        hwmon.pwms = init_sensors(&hwmon, 1)?;
        hwmon.temps = init_sensors(&hwmon, 1)?;
        hwmon.voltages = init_sensors(&hwmon, 0)?;

        Ok(hwmon)
    }
}

/// The read/write variant of Hwmon. It contains all sensors found whithin its directory path.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct ReadWriteHwmon {
    name: String,
    path: PathBuf,
    currs: BTreeMap<u16, ReadWriteCurr>,
    energies: BTreeMap<u16, ReadWriteEnergy>,
    fans: BTreeMap<u16, ReadWriteFan>,
    humidities: BTreeMap<u16, ReadWriteHumidity>,
    powers: BTreeMap<u16, ReadWritePower>,
    pwms: BTreeMap<u16, ReadWritePwm>,
    temps: BTreeMap<u16, ReadWriteTemp>,
    voltages: BTreeMap<u16, ReadWriteVolt>,
}

#[cfg(feature = "writable")]
impl ReadWriteHwmon {
    /// Returns all current sensors found in this hwmon.
    pub fn currents(&self) -> &BTreeMap<u16, ReadWriteCurr> {
        &self.currs
    }

    /// Returns all energy sensors found in this hwmon.
    pub fn energies(&self) -> &BTreeMap<u16, ReadWriteEnergy> {
        &self.energies
    }

    /// Returns all fan sensors found in this hwmon.
    pub fn fans(&self) -> &BTreeMap<u16, ReadWriteFan> {
        &self.fans
    }

    /// Returns all humidity sensors found in this hwmon.
    pub fn humidities(&self) -> &BTreeMap<u16, ReadWriteHumidity> {
        &self.humidities
    }

    /// Returns all power sensors found in this hwmon.
    pub fn powers(&self) -> &BTreeMap<u16, ReadWritePower> {
        &self.powers
    }

    /// Returns all pwm sensors found in this hwmon.
    pub fn pwms(&self) -> &BTreeMap<u16, ReadWritePwm> {
        &self.pwms
    }

    /// Returns all temp sensors found in this hwmon.
    pub fn temps(&self) -> &BTreeMap<u16, ReadWriteTemp> {
        &self.temps
    }

    /// Returns all voltage sensors found in this hwmon.
    pub fn voltages(&self) -> &BTreeMap<u16, ReadWriteVolt> {
        &self.voltages
    }
}

#[cfg(feature = "writable")]
impl Hwmon for ReadWriteHwmon {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &Path {
        &self.path.as_path()
    }
}

#[cfg(feature = "writable")]
impl Parseable for ReadWriteHwmon {
    type Parent = Hwmons<Self>;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let path = parent.path().join(format!("hwmon{}", index));

        check_path(&path)?;

        let mut hwmon = Self {
            name: get_name(&path)?,
            path,
            currs: BTreeMap::new(),
            energies: BTreeMap::new(),
            fans: BTreeMap::new(),
            humidities: BTreeMap::new(),
            powers: BTreeMap::new(),
            pwms: BTreeMap::new(),
            temps: BTreeMap::new(),
            voltages: BTreeMap::new(),
        };

        hwmon.currs = init_sensors(&hwmon, 1)?;
        hwmon.energies = init_sensors(&hwmon, 1)?;
        hwmon.fans = init_sensors(&hwmon, 1)?;
        hwmon.humidities = init_sensors(&hwmon, 1)?;
        hwmon.powers = init_sensors(&hwmon, 1)?;
        hwmon.pwms = init_sensors(&hwmon, 1)?;
        hwmon.temps = init_sensors(&hwmon, 1)?;
        hwmon.voltages = init_sensors(&hwmon, 0)?;

        Ok(hwmon)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::*;
    use crate::*;

    use std::fs::remove_dir_all;
    use std::path::Path;

    #[test]
    fn test_hwmon_parse() {
        let test_path = Path::new("test_hwmon_parse");

        VirtualHwmonBuilder::create(test_path, 0, "system");

        let hwmons: Hwmons<ReadOnlyHwmon> = parse(test_path).unwrap();
        let hwmon = hwmons.get_hwmon_by_index(&0).unwrap();

        assert_eq!("system", hwmon.name());
        assert_eq!(test_path.join("hwmon0"), hwmon.path());

        remove_dir_all(test_path).unwrap();
    }

    #[test]
    fn test_hwmon_temps() {
        let test_path = Path::new("test_hwmon_init");

        VirtualHwmonBuilder::create(test_path, 0, "system")
            .add_temp(1, 40000, "temp1")
            .add_temp(2, 60000, "temp2");

        let hwmons: Hwmons<ReadOnlyHwmon> = parse(test_path).unwrap();
        let hwmon = hwmons.get_hwmon_by_index(&0).unwrap();
        let temps = hwmon.temps();

        temps.get(&1u16).unwrap();
        temps.get(&2u16).unwrap();

        assert_eq!(true, temps.get(&3u16).is_none());

        remove_dir_all(test_path).unwrap();
    }

    #[test]
    fn test_hwmon_pwms() {
        let test_path = Path::new("test_hwmon_pwms");

        VirtualHwmonBuilder::create(test_path, 0, "system")
            .add_pwm(1, true, true)
            .add_pwm(2, true, true);

        let hwmons: Hwmons<ReadOnlyHwmon> = parse(test_path).unwrap();
        let hwmon = hwmons.get_hwmon_by_index(&0).unwrap();
        let pwms = hwmon.pwms();

        pwms.get(&1u16).unwrap();
        pwms.get(&2u16).unwrap();

        assert_eq!(true, pwms.get(&3u16).is_none());

        remove_dir_all(test_path).unwrap();
    }
}
