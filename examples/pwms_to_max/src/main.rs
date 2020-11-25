use libmedium::{
    parse_hwmons,
    sensors::WriteablePwmSensor,
    units::{Pwm, PwmEnable},
};

fn main() {
    let hwmons = parse_hwmons().unwrap();
    for (_, _, hwmon) in &hwmons {
        for (_, pwm) in hwmon.writeable_pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
        }
    }
}
