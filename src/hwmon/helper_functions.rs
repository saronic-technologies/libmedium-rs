use super::*;

pub(crate) fn check_path(path: impl AsRef<Path>) -> ParsingResult<()> {
    let path = path.as_ref();

    if let Err(e) = path.metadata() {
        return Err(ParsingError::hwmon_dir(e, path));
    }

    Ok(())
}

pub(crate) fn get_name(path: impl AsRef<Path>) -> ParsingResult<String> {
    let name_path = path.as_ref().join("name");

    read_to_string(&name_path)
        .map(|name| name.trim().to_string())
        .map_err(|e| ParsingError::hwmon_name(e, name_path))
}

pub(crate) fn init_sensors<S>(hwmon: &Hwmon, start_index: u16) -> ParsingResult<BTreeMap<u16, S>>
where
    S: Parseable<Parent = Hwmon>,
{
    use std::io::ErrorKind as IoErrorKind;

    let mut sensors = BTreeMap::new();
    for index in start_index..=u16::MAX {
        match S::parse(hwmon, index) {
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
