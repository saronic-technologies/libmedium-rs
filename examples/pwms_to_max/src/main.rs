use libmedium::{
    parse_hwmons,
    sensors::pwm::WriteablePwmSensor,
    units::{Pwm, PwmEnable},
};

fn main() {
    let hwmons = parse_hwmons().unwrap();
    for hwmon in &hwmons {
        for (_, pwm) in hwmon.writeable_pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::from_u8(255)).unwrap();
        }
    }
}
