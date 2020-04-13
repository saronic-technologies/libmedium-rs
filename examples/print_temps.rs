use libmedium::hwmon::Hwmon;
use libmedium::parse_hwmons_read_only;
use libmedium::sensors::{Sensor, SensorBase};

fn main() {
    let hwmons = parse_hwmons_read_only().unwrap();
    for (hwmon_index, hwmon_name, hwmon) in &hwmons {
        println!("hwmon{} with name {}:", hwmon_index, hwmon_name);
        for (_, temp_sensor) in hwmon.temps() {
            let temperature = temp_sensor.read_input().unwrap();
            println!("\t{}: {}", temp_sensor.name(), temperature);
        }
    }
}
