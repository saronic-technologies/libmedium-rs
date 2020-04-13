use libmedium::hwmon::Hwmon;
use libmedium::parse_hwmons_read_write;
use libmedium::sensors::pwm::{Pwm, PwmEnable, PwmSensor};

fn main() {
    let hwmons = parse_hwmons_read_write().unwrap();
    for (_, _, hwmon) in &hwmons {
        for (_, pwm) in hwmon.pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
        }
    }
}
