//! Module containing the Hwmon struct and related functionality.

mod hwmon;
mod hwmons;

pub use hwmon::Hwmon;
pub use hwmons::Hwmons;

pub use crate::parsing::Error as ParsingError;

#[cfg(test)]
pub mod tests {
    use crate::hwmon::Hwmons;
    use crate::tests::*;

    use temp_dir::TempDir;

    #[test]
    fn test_hwmon_parse() {
        let test_dir = TempDir::new().unwrap();

        VirtualHwmonBuilder::create(test_dir.path(), 0, "system");

        let hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();

        assert_eq!("system", hwmon.name());
        assert_eq!(test_dir.path().join("hwmon0"), hwmon.path());
    }

    #[test]
    fn test_hwmon_temps() {
        let test_dir = TempDir::new().unwrap();

        VirtualHwmonBuilder::create(test_dir.path(), 0, "system")
            .add_temp(1, 40000, "temp1")
            .add_temp(2, 60000, "temp2")
            .add_temp(4, 30000, "temp4");

        let hwmons: Hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let temps = hwmon.temps();

        assert_eq!(true, temps.get(&1u16).is_some());
        assert_eq!(true, temps.get(&2u16).is_some());
        assert_eq!(true, temps.get(&4u16).is_some());

        assert_eq!(true, temps.get(&3u16).is_none());
    }

    #[test]
    fn test_hwmon_pwms() {
        let test_dir = TempDir::new().unwrap();

        VirtualHwmonBuilder::create(test_dir.path(), 0, "system")
            .add_pwm(1, true, true)
            .add_pwm(2, true, true)
            .add_pwm(4, true, true);

        let hwmons: Hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let pwms = hwmon.pwms();

        assert_eq!(true, pwms.get(&1u16).is_some());
        assert_eq!(true, pwms.get(&2u16).is_some());
        assert_eq!(true, pwms.get(&4u16).is_some());

        assert_eq!(true, pwms.get(&3u16).is_none());
    }
}
