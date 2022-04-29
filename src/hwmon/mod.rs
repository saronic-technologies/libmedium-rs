//! Module containing the Hwmon struct and related functionality.

mod error;
mod helper_functions;
mod iterator;

pub use error::Error;
pub use iterator::{Iter, NamedIter};

use error::Result;
use helper_functions::*;

use crate::parsing::{Error as ParsingError, Parseable, Result as ParsingResult};
use crate::sensors::{
    curr::*, energy::*, fan::*, humidity::*, intrusion::*, power::*, pwm::*, temp::*, voltage::*,
};

use crate::units::Raw;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::Debug,
    fs::read_to_string,
    io::ErrorKind as IoErrorKind,
    path::{Path, PathBuf},
    time::Duration,
};

/// Struct representing a hwmon directory.
#[derive(Debug, Clone)]
pub struct Hwmon {
    name: String,
    path: PathBuf,
    index: u16,
    currents: BTreeMap<u16, CurrentSensorStruct>,
    energies: BTreeMap<u16, EnergySensorStruct>,
    fans: BTreeMap<u16, FanSensorStruct>,
    humidities: BTreeMap<u16, HumiditySensorStruct>,
    intrusions: BTreeMap<u16, IntrusionSensorStruct>,
    powers: BTreeMap<u16, PowerSensorStruct>,
    pwms: BTreeMap<u16, PwmSensorStruct>,
    temps: BTreeMap<u16, TempSensorStruct>,
    voltages: BTreeMap<u16, VoltageSensorStruct>,
}

impl Hwmon {
    /// Returns the hwmon's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the hwmon's path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// returns the hwmon's index.
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Returns this hwmon's device path.
    /// This path does not change between reboots.
    pub fn device_path(&self) -> PathBuf {
        // Every hwmon in sysfs has a device link so this should never panic.
        self.path().join("device").canonicalize().unwrap()
    }

    /// Returns this hwmon's update interval.
    /// If the hwmon does not expose the value, an error is returned.
    pub fn update_interval(&self) -> Result<Duration> {
        let path = self.path().join("update_interval");

        match std::fs::read_to_string(&path) {
            Ok(s) => Duration::from_raw(&s).map_err(|e| Error::unit(e, path)),
            Err(e) => {
                if e.kind() == IoErrorKind::NotFound {
                    Err(Error::update_interval_not_available())
                } else {
                    Err(Error::io(e, path))
                }
            }
        }
    }

    /// Returns whether this hwmon beeps if an alarm condition exists.
    /// If the hwmon does not expose the value, an error is returned.
    pub fn beep_enable(&self) -> Result<bool> {
        let path = self.path().join("beep_enable");

        match std::fs::read_to_string(&path) {
            Ok(s) => bool::from_raw(&s).map_err(|e| Error::unit(e, path)),
            Err(e) => {
                if e.kind() == IoErrorKind::NotFound {
                    Err(Error::beep_enable())
                } else {
                    Err(Error::io(e, path))
                }
            }
        }
    }

    /// Returns all current sensors found in this `Hwmon`.
    pub fn currents(&self) -> &BTreeMap<u16, impl CurrentSensor + Clone + Send + Sync> {
        &self.currents
    }

    /// Returns all energy sensors found in this `Hwmon`.
    pub fn energies(&self) -> &BTreeMap<u16, impl EnergySensor + Clone + Send + Sync> {
        &self.energies
    }

    /// Returns all fan sensors found in this `Hwmon`.
    pub fn fans(&self) -> &BTreeMap<u16, impl FanSensor + Clone + Send + Sync> {
        &self.fans
    }

    /// Returns all humidity sensors found in this `Hwmon`.
    pub fn humidities(&self) -> &BTreeMap<u16, impl HumiditySensor + Clone + Send + Sync> {
        &self.humidities
    }

    /// Returns all intrusion sensors found in this `Hwmon`.
    pub fn intrusions(&self) -> &BTreeMap<u16, impl IntrusionSensor + Clone + Send + Sync> {
        &self.intrusions
    }

    /// Returns all power sensors found in this `Hwmon`.
    pub fn powers(&self) -> &BTreeMap<u16, impl PowerSensor + Clone + Send + Sync> {
        &self.powers
    }

    /// Returns all pwm sensors found in this `Hwmon`.
    pub fn pwms(&self) -> &BTreeMap<u16, impl PwmSensor + Clone + Send + Sync> {
        &self.pwms
    }

    /// Returns all temp sensors found in this `Hwmon`.
    pub fn temps(&self) -> &BTreeMap<u16, impl TempSensor + Clone + Send + Sync> {
        &self.temps
    }

    /// Returns all voltage sensors found in this `Hwmon`.
    pub fn voltages(&self) -> &BTreeMap<u16, impl VoltageSensor + Clone + Send + Sync> {
        &self.voltages
    }

    /// Returns the current sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn current(&self, index: u16) -> Option<&(impl CurrentSensor + Clone + Send + Sync)> {
        self.currents.get(&index)
    }

    /// Returns the energy sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn energy(&self, index: u16) -> Option<&(impl EnergySensor + Clone + Send + Sync)> {
        self.energies.get(&index)
    }

    /// Returns the fan sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn fan(&self, index: u16) -> Option<&(impl FanSensor + Clone + Send + Sync)> {
        self.fans.get(&index)
    }

    /// Returns the humidity sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn humidity(&self, index: u16) -> Option<&(impl HumiditySensor + Clone + Send + Sync)> {
        self.humidities.get(&index)
    }

    /// Returns the intrusion sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn intrusion(&self, index: u16) -> Option<&(impl IntrusionSensor + Clone + Send + Sync)> {
        self.intrusions.get(&index)
    }

    /// Returns the power sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn power(&self, index: u16) -> Option<&(impl PowerSensor + Clone + Send + Sync)> {
        self.powers.get(&index)
    }

    /// Returns the pwm sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn pwm(&self, index: u16) -> Option<&(impl PwmSensor + Clone + Send + Sync)> {
        self.pwms.get(&index)
    }

    /// Returns the temp sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn temp(&self, index: u16) -> Option<&(impl TempSensor + Clone + Send + Sync)> {
        self.temps.get(&index)
    }

    /// Returns the voltage sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn voltage(&self, index: u16) -> Option<&(impl VoltageSensor + Clone + Send + Sync)> {
        self.voltages.get(&index)
    }

    pub(crate) fn try_from_path(path: impl Into<PathBuf>, index: u16) -> ParsingResult<Self> {
        let path = path.into();

        check_path(&path)?;

        let mut hwmon = Self {
            name: get_name(&path)?,
            path,
            index,
            currents: BTreeMap::new(),
            energies: BTreeMap::new(),
            fans: BTreeMap::new(),
            humidities: BTreeMap::new(),
            intrusions: BTreeMap::new(),
            powers: BTreeMap::new(),
            pwms: BTreeMap::new(),
            temps: BTreeMap::new(),
            voltages: BTreeMap::new(),
        };

        hwmon.currents = init_sensors(&hwmon, 1)?;
        hwmon.energies = init_sensors(&hwmon, 1)?;
        hwmon.fans = init_sensors(&hwmon, 1)?;
        hwmon.humidities = init_sensors(&hwmon, 1)?;
        hwmon.intrusions = init_sensors(&hwmon, 0)?;
        hwmon.powers = init_sensors(&hwmon, 1)?;
        hwmon.pwms = init_sensors(&hwmon, 1)?;
        hwmon.temps = init_sensors(&hwmon, 1)?;
        hwmon.voltages = init_sensors(&hwmon, 0)?;

        Ok(hwmon)
    }
}

#[cfg(feature = "writeable")]
impl Hwmon {
    /// Set this hwmon's update interval.
    /// If the hwmon does not expose the value, an error is returned.
    pub fn set_update_interval(&self, interval: Duration) -> Result<()> {
        let path = self.path().join("update_interval");

        match std::fs::write(&path, interval.to_raw().as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                IoErrorKind::NotFound => Err(Error::update_interval_not_available()),
                IoErrorKind::PermissionDenied => Err(Error::insufficient_rights(path)),
                _ => Err(Error::io(e, path)),
            },
        }
    }

    /// Set whether this hwmon beeps if an alarm condition exists.
    /// If the hwmon does not expose the value, an error is returned.
    pub fn set_beep_enable(&self, beep_enable: bool) -> Result<()> {
        let path = self.path().join("beep_enable");

        match std::fs::write(&path, beep_enable.to_raw().as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                IoErrorKind::NotFound => Err(Error::beep_enable()),
                IoErrorKind::PermissionDenied => Err(Error::insufficient_rights(path)),
                _ => Err(Error::io(e, path)),
            },
        }
    }

    /// Returns all writeable current sensors found in this `Hwmon`.
    pub fn writeable_currents(
        &self,
    ) -> &BTreeMap<u16, impl WriteableCurrentSensor + Clone + Send + Sync> {
        &self.currents
    }

    /// Returns all writeable energy sensors found in this `Hwmon`.
    pub fn writeable_energies(
        &self,
    ) -> &BTreeMap<u16, impl WriteableEnergySensor + Clone + Send + Sync> {
        &self.energies
    }

    /// Returns all writeable fan sensors found in this `Hwmon`.
    pub fn writeable_fans(&self) -> &BTreeMap<u16, impl WriteableFanSensor + Clone + Send + Sync> {
        &self.fans
    }

    /// Returns all writeable humidity sensors found in this `Hwmon`.
    pub fn writeable_humidities(
        &self,
    ) -> &BTreeMap<u16, impl WriteableHumiditySensor + Clone + Send + Sync> {
        &self.humidities
    }

    /// Returns all writeable intrusion sensors found in this `Hwmon`.
    pub fn writeable_intrusions(
        &self,
    ) -> &BTreeMap<u16, impl WriteableIntrusionSensor + Clone + Send + Sync> {
        &self.intrusions
    }

    /// Returns all writeable power sensors found in this `Hwmon`.
    pub fn writeable_powers(
        &self,
    ) -> &BTreeMap<u16, impl WriteablePowerSensor + Clone + Send + Sync> {
        &self.powers
    }

    /// Returns all writeable pwm sensors found in this `Hwmon`.
    pub fn writeable_pwms(&self) -> &BTreeMap<u16, impl WriteablePwmSensor + Clone + Send + Sync> {
        &self.pwms
    }

    /// Returns all writeable temp sensors found in this `Hwmon`.
    pub fn writeable_temps(
        &self,
    ) -> &BTreeMap<u16, impl WriteableTempSensor + Clone + Send + Sync> {
        &self.temps
    }

    /// Returns all writeable voltage sensors found in this `Hwmon`.
    pub fn writeable_voltages(
        &self,
    ) -> &BTreeMap<u16, impl WriteableVoltageSensor + Clone + Send + Sync> {
        &self.voltages
    }

    /// Returns the writeable current sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_current(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableCurrentSensor + Clone + Send + Sync)> {
        self.currents.get(&index)
    }

    /// Returns the writeable energy sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_energy(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableEnergySensor + Clone + Send + Sync)> {
        self.energies.get(&index)
    }

    /// Returns the writeable fan sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_fan(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableFanSensor + Clone + Send + Sync)> {
        self.fans.get(&index)
    }

    /// Returns the writeable humidity sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_humidity(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableHumiditySensor + Clone + Send + Sync)> {
        self.humidities.get(&index)
    }

    /// Returns the writeable intrusion sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_intrusion(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableIntrusionSensor + Clone + Send + Sync)> {
        self.intrusions.get(&index)
    }

    /// Returns the writeable power sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_power(
        &self,
        index: u16,
    ) -> Option<&(impl WriteablePowerSensor + Clone + Send + Sync)> {
        self.powers.get(&index)
    }

    /// Returns the writeable pwm sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_pwm(
        &self,
        index: u16,
    ) -> Option<&(impl WriteablePwmSensor + Clone + Send + Sync)> {
        self.pwms.get(&index)
    }

    /// Returns the writeable temp sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_temp(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableTempSensor + Clone + Send + Sync)> {
        self.temps.get(&index)
    }

    /// Returns the writeable voltage sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    pub fn writeable_voltage(
        &self,
        index: u16,
    ) -> Option<&(impl WriteableVoltageSensor + Clone + Send + Sync)> {
        self.voltages.get(&index)
    }
}

impl PartialEq for Hwmon {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(other.path())
    }
}

impl Eq for Hwmon {}

impl PartialOrd for Hwmon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(other.path())
    }
}

impl Ord for Hwmon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl Parseable for Hwmon {
    type Parent = Hwmons;

    fn parse(parent: &Self::Parent, index: u16) -> ParsingResult<Self> {
        let path = parent.path().join(format!("hwmon{}", index));

        Self::try_from_path(path, index)
    }
}

/// This crate's central struct.
/// It stores all parsed [`Hwmon`](crate::hwmon::Hwmon)s which you can query either by name, device path or index.
#[derive(Debug, Clone)]
pub struct Hwmons {
    path: PathBuf,
    hwmons: BTreeMap<u16, Hwmon>,
}

impl Hwmons {
    /// Parses /sys/class/hwmon and returns the found hwmons as a Hwmons object.
    pub fn parse() -> ParsingResult<Self> {
        Self::parse_path("/sys/class/hwmon/")
    }

    /// The path that was parsed to generate this object.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns an iterator over all hwmons with the given name and their indices.
    /// Returns an empty iterator, if there is no `Hwmon` with the given name.
    pub fn hwmons_by_name<N: AsRef<str>>(&self, name: N) -> NamedIter<N> {
        NamedIter::new(self.iter(), name)
    }

    /// Get a `Hwmon` by its index.
    /// Returns `None`, if there is no `Hwmon` with the given index.
    pub fn hwmon_by_index(&self, index: u16) -> Option<&Hwmon> {
        self.hwmons.get(&index)
    }

    /// Get a `Hwmon` by its device path.
    /// Returns `None`, if there is no `Hwmon` with the given device path.
    pub fn hwmon_by_device_path(&self, device_path: impl AsRef<Path>) -> Option<&Hwmon> {
        self.hwmons
            .values()
            .find(move |&hwmon| hwmon.device_path() == device_path.as_ref())
    }

    /// Returns an iterator over all hwmons, their names and their indices.
    pub fn iter(&self) -> Iter {
        Iter::new(self.hwmons.iter())
    }

    /// Parses the provided path and returns the found hwmons as a Hwmons object.
    #[cfg(feature = "unrestricted_parsing")]
    pub fn parse_unrestricted(path: impl AsRef<Path>) -> ParsingResult<Self> {
        Self::parse_path(path)
    }

    pub(crate) fn parse_path(path: impl AsRef<Path>) -> ParsingResult<Self> {
        let path = path.as_ref();

        let mut hwmons = Hwmons {
            path: path.to_path_buf(),
            hwmons: BTreeMap::new(),
        };

        let mut index;

        for entry in path.read_dir().map_err(|e| ParsingError::hwmons(e, path))? {
            let entry = entry.map_err(|e| ParsingError::hwmons(e, path))?;

            match entry.file_name().to_str() {
                Some(file_name) => match file_name.strip_prefix("hwmon") {
                    Some(rest) => {
                        index = rest
                            .parse()
                            .map_err(|e| ParsingError::hwmon_index(e, path))?;
                    }
                    None => continue,
                },
                None => continue,
            }

            hwmons
                .hwmons
                .insert(index, Hwmon::try_from_path(entry.path(), index)?);
        }

        Ok(hwmons)
    }
}

#[cfg(test)]
mod tests;
