use libmedium::{
    parse_hwmons,
    sensors::{temp::TempSensor, Sensor},
};

fn main() {
    let hwmons = parse_hwmons().unwrap();
    for hwmon in &hwmons {
        println!("hwmon{} with name {}:", hwmon.index(), hwmon.name());
        for (_, temp_sensor) in hwmon.temps() {
            let temperature = temp_sensor.read_input().unwrap();
            println!("\t{}: {:?}", temp_sensor.name(), temperature);
        }
    }
}
