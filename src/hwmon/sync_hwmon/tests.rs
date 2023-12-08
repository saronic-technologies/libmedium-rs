use super::Hwmons;

use crate::tests::*;
use std::time::Duration;

use temp_dir::TempDir;

#[test]
fn test_hwmon_parse() {
    let test_dir = TempDir::new().unwrap();

    VirtualHwmonBuilder::create(test_dir.path(), 0, "foo");
    VirtualHwmonBuilder::create(test_dir.path(), 1, "bar");

    let hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
    let foo = hwmons.hwmon_by_index(0).unwrap();
    let bar = hwmons.hwmon_by_index(1).unwrap();

    assert_eq!("foo", foo.name());
    assert_eq!(Duration::from_secs(1), foo.update_interval().unwrap());
    assert_eq!(test_dir.path().join("hwmon0"), foo.path());
    assert_eq!("bar", bar.name());
    assert_eq!(test_dir.path().join("hwmon1"), bar.path());
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
