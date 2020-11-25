//! Module containing the Hwmon struct and related functionality.

mod hwmon;
mod hwmons;

pub use hwmon::Hwmon;
pub use hwmons::Hwmons;

#[cfg(test)]
pub mod tests {
    use crate::hwmon::Hwmons;
    use crate::tests::*;

    use std::fs::remove_dir_all;
    use std::path::Path;

    #[test]
    fn test_hwmon_parse() {
        let test_path = Path::new("test_hwmon_parse");

        VirtualHwmonBuilder::create(test_path, 0, "system");

        let hwmons = Hwmons::parse_path(test_path).unwrap();
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

        let hwmons: Hwmons = Hwmons::parse_path(test_path).unwrap();
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

        let hwmons: Hwmons = Hwmons::parse_path(test_path).unwrap();
        let hwmon = hwmons.hwmon_by_index(0).unwrap();
        let pwms = hwmon.pwms();

        pwms.get(&1u16).unwrap();
        pwms.get(&2u16).unwrap();

        assert_eq!(true, pwms.get(&3u16).is_none());

        remove_dir_all(test_path).unwrap();
    }
}
