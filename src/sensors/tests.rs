use super::shared_subfunctions::*;
use super::*;
use crate::hwmon::Hwmons;
use crate::parsing::Parseable;
use crate::sensors::TempSensorStruct;
use crate::tests::*;

use temp_dir::TempDir;

#[test]
fn test_sensor_read_value() {
    let test_dir = TempDir::new().unwrap();

    VirtualHwmonBuilder::create(test_dir.path(), 0, "system")
        .add_temp(1, 40000, "temp1")
        .add_fan(1, 60);

    let hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
    let hwmon = hwmons.hwmon_by_index(0).unwrap();
    let temp = TempSensorStruct::parse(hwmon, 1).unwrap();
    let fan = FanSensorStruct::parse(hwmon, 1).unwrap();

    #[cfg(not(feature = "uom_units"))]
    assert_eq!(40.0, temp.read_input().unwrap().as_degrees_celsius());

    #[cfg(feature = "uom_units")]
    assert_eq!(
        40.0,
        temp.read_input()
            .unwrap()
            .round::<uom::si::thermodynamic_temperature::degree_celsius>()
            .get::<uom::si::thermodynamic_temperature::degree_celsius>()
    );

    #[cfg(not(feature = "uom_units"))]
    assert_eq!(60, fan.read_input().unwrap().as_rpm());

    #[cfg(feature = "uom_units")]
    assert_eq!(
        60.0,
        fan.read_input()
            .unwrap()
            .round::<uom::si::angular_velocity::revolution_per_minute>()
            .get::<uom::si::angular_velocity::revolution_per_minute>()
    );
}

#[test]
fn test_label() {
    let test_dir = TempDir::new().unwrap();

    VirtualHwmonBuilder::create(test_dir.path(), 0, "system").add_temp(1, 40000, "test_temp1\n");

    let hwmons: Hwmons = Hwmons::parse_path(test_dir.path()).unwrap();
    let hwmon = hwmons.hwmon_by_index(0).unwrap();
    let temp = TempSensorStruct::parse(hwmon, 1).unwrap();

    assert_eq!(temp.name(), String::from("test_temp1"));
}
