use libmedium::{
    parse_hwmons,
    sensors::sync_sensors::pwm::WriteablePwmSensor,
    units::{Pwm, PwmEnable},
};

fn main() {
    let hwmons = parse_hwmons().unwrap();
    for hwmon in &hwmons {
        for (_, pwm) in hwmon.writeable_pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::FULLSPEED).unwrap();
        }
    }
}
