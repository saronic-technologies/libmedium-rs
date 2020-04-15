//! Module containing the Hwmon struct and related functionality.

use super::{Hwmons, Parseable, ParsingError, ParsingResult};
use crate::sensors::*;

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
            std::io::ErrorKind::NotFound => {
                return Err(ParsingError::PathDoesNotExist { path: path.into() })
            }
            std::io::ErrorKind::PermissionDenied => {
                return Err(ParsingError::InsufficientRights { path: path.into() })
            }
            _ => return Err(ParsingError::Other { source: e }),
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
                return Err(ParsingError::PathDoesNotExist { path: name_path })
            }
            std::io::ErrorKind::PermissionDenied => {
                return Err(ParsingError::InsufficientRights { path: name_path })
            }
            _ => return Err(ParsingError::Other { source: e }),
        },
    };

    Ok(name)
}

fn init_sensors<S, H>(hwmon: &H, start_index: u16) -> ParsingResult<BTreeMap<u16, S>>
where
    S: SensorBase + Parseable<Parent = H>,
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
    /// The type of current sensor this `Hwmon` supports.
    type Current: CurrSensor;

    /// The type of energy sensor this `Hwmon` supports.
    type Energy: EnergySensor;

    /// The type of fan sensor this `Hwmon` supports.
    type Fan: FanSensor;

    /// The type of humidity sensor this `Hwmon` supports.
    type Humidity: HumiditySensor;

    /// The type of power sensor this `Hwmon` supports.
    type Power: PowerSensor;

    /// The type of pwm sensor this `Hwmon` supports.
    type Pwm: PwmSensor;

    /// The type of temp sensor this `Hwmon` supports.
    type Temp: TempSensor;

    /// The type of voltage sensor this `Hwmon` supports.
    type Voltage: VoltSensor;

    /// Returns the hwmon's name.
    fn name(&self) -> &str;

    /// Returns the hwmon's path.
    fn path(&self) -> &Path;

    /// Returns all current sensors found in this `Hwmon`.
    fn currents(&self) -> &BTreeMap<u16, Self::Current>;

    /// Returns all energy sensors found in this `Hwmon`.
    fn energies(&self) -> &BTreeMap<u16, Self::Energy>;

    /// Returns all fan sensors found in this `Hwmon`.
    fn fans(&self) -> &BTreeMap<u16, Self::Fan>;

    /// Returns all humidity sensors found in this `Hwmon`.
    fn humidities(&self) -> &BTreeMap<u16, Self::Humidity>;

    /// Returns all power sensors found in this `Hwmon`.
    fn powers(&self) -> &BTreeMap<u16, Self::Power>;

    /// Returns all pwm sensors found in this `Hwmon`.
    fn pwms(&self) -> &BTreeMap<u16, Self::Pwm>;

    /// Returns all temp sensors found in this `Hwmon`.
    fn temps(&self) -> &BTreeMap<u16, Self::Temp>;

    /// Returns all voltage sensors found in this `Hwmon`.
    fn voltages(&self) -> &BTreeMap<u16, Self::Voltage>;

    /// Returns the current sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn current(&self, index: u16) -> Option<&Self::Current> {
        self.currents().get(&index)
    }

    /// Returns the energy sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn energy(&self, index: u16) -> Option<&Self::Energy> {
        self.energies().get(&index)
    }

    /// Returns the fan sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn fan(&self, index: u16) -> Option<&Self::Fan> {
        self.fans().get(&index)
    }

    /// Returns the humidity sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn humidity(&self, index: u16) -> Option<&Self::Humidity> {
        self.humidities().get(&index)
    }

    /// Returns the power sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn power(&self, index: u16) -> Option<&Self::Power> {
        self.powers().get(&index)
    }

    /// Returns the pwm sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn pwm(&self, index: u16) -> Option<&Self::Pwm> {
        self.pwms().get(&index)
    }

    /// Returns the temp sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn temp(&self, index: u16) -> Option<&Self::Temp> {
        self.temps().get(&index)
    }

    /// Returns the voltage sensor with the given index.
    /// Returns `None`, if no sensor with the given index exists.
    fn voltage(&self, index: u16) -> Option<&Self::Voltage> {
        self.voltages().get(&index)
    }
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

impl Hwmon for ReadOnlyHwmon {
    type Current = ReadOnlyCurr;
    type Energy = ReadOnlyEnergy;
    type Fan = ReadOnlyFan;
    type Humidity = ReadOnlyHumidity;
    type Power = ReadOnlyPower;
    type Pwm = ReadOnlyPwm;
    type Temp = ReadOnlyTemp;
    type Voltage = ReadOnlyVolt;

    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &Path {
        &self.path.as_path()
    }

    fn currents(&self) -> &BTreeMap<u16, Self::Current> {
        &self.currs
    }

    fn energies(&self) -> &BTreeMap<u16, Self::Energy> {
        &self.energies
    }

    fn fans(&self) -> &BTreeMap<u16, Self::Fan> {
        &self.fans
    }

    fn humidities(&self) -> &BTreeMap<u16, Self::Humidity> {
        &self.humidities
    }

    fn powers(&self) -> &BTreeMap<u16, Self::Power> {
        &self.powers
    }

    fn pwms(&self) -> &BTreeMap<u16, Self::Pwm> {
        &self.pwms
    }

    fn temps(&self) -> &BTreeMap<u16, Self::Temp> {
        &self.temps
    }

    fn voltages(&self) -> &BTreeMap<u16, Self::Voltage> {
        &self.voltages
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

#[cfg(feature = "writable")]
impl From<ReadWriteHwmon> for ReadOnlyHwmon {
    fn from(read_write: ReadWriteHwmon) -> Self {
        ReadOnlyHwmon {
            name: read_write.name,
            path: read_write.path,
            currs: read_write
                .currs
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            energies: read_write
                .energies
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            fans: read_write
                .fans
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            humidities: read_write
                .humidities
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            powers: read_write
                .powers
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            pwms: read_write
                .pwms
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            temps: read_write
                .temps
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
            voltages: read_write
                .voltages
                .into_iter()
                .map(|(i, s)| (i, s.into()))
                .collect(),
        }
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
impl Hwmon for ReadWriteHwmon {
    type Current = ReadWriteCurr;
    type Energy = ReadWriteEnergy;
    type Fan = ReadWriteFan;
    type Humidity = ReadWriteHumidity;
    type Power = ReadWritePower;
    type Pwm = ReadWritePwm;
    type Temp = ReadWriteTemp;
    type Voltage = ReadWriteVolt;

    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> &Path {
        &self.path.as_path()
    }

    fn currents(&self) -> &BTreeMap<u16, Self::Current> {
        &self.currs
    }

    fn energies(&self) -> &BTreeMap<u16, Self::Energy> {
        &self.energies
    }

    fn fans(&self) -> &BTreeMap<u16, Self::Fan> {
        &self.fans
    }

    fn humidities(&self) -> &BTreeMap<u16, Self::Humidity> {
        &self.humidities
    }

    fn powers(&self) -> &BTreeMap<u16, Self::Power> {
        &self.powers
    }

    fn pwms(&self) -> &BTreeMap<u16, Self::Pwm> {
        &self.pwms
    }

    fn temps(&self) -> &BTreeMap<u16, Self::Temp> {
        &self.temps
    }

    fn voltages(&self) -> &BTreeMap<u16, Self::Voltage> {
        &self.voltages
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

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();

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

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
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

        let hwmons: Hwmons<ReadOnlyHwmon> = Hwmons::parse(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let pwms = hwmon.pwms();

        pwms.get(&1u16).unwrap();
        pwms.get(&2u16).unwrap();

        assert_eq!(true, pwms.get(&3u16).is_none());

        remove_dir_all(test_path).unwrap();
    }
}
