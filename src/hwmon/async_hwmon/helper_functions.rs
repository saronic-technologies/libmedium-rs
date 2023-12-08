use super::*;

use crate::parsing::{Error as ParsingError, Result as ParsingResult};

use std::path::Path;
use std::io::ErrorKind as IoErrorKind;

pub(crate) fn check_path(path: impl AsRef<Path>) -> ParsingResult<()> {
    let path = path.as_ref();

    if let Err(e) = path.metadata() {
        return Err(ParsingError::hwmon_dir(e, path));
    }

    Ok(())
}


pub(crate) async fn get_name(path: impl AsRef<Path>) -> ParsingResult<String> {
    let name_path = path.as_ref().join("name");

    tokio::fs::read_to_string(&name_path).await
        .map(|name| name.trim().to_string())
        .map_err(|e| ParsingError::hwmon_name(e, name_path))
}

pub(crate) async fn init_sensors<S>(hwmon: &Hwmon, start_index: u16) -> ParsingResult<BTreeMap<u16, S>>
where
    S: AsyncParseable<Parent = Hwmon>,
{
    let mut stop_index = start_index;

    let dir = hwmon
        .path()
        .read_dir()
        .map_err(|e| ParsingError::hwmon_dir(e, hwmon.path()))?;

    for entry in dir {
        let entry = entry.map_err(|e| ParsingError::hwmon_dir(e, hwmon.path()))?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        if !file_name.starts_with(S::prefix()) {
            continue;
        }

        let index = file_name
            .trim_matches(|ch: char| !ch.is_ascii_digit())
            .parse()
            .unwrap_or(0);

        if index > stop_index {
            stop_index = index;
        }
    }

    let mut sensors = BTreeMap::new();

    for index in start_index..=stop_index {
        match S::parse(hwmon, index).await {
            Ok(sensor) => {
                sensors.insert(index, sensor);
            }
            Err(e) => match &e {
                ParsingError::Sensor { source, .. } => {
                    if source.kind() != IoErrorKind::NotFound {
                        return Err(e);
                    }
                }
                _ => return Err(e),
            },
        }
    }

    Ok(sensors)
}
