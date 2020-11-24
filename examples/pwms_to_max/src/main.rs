use libmedium::{
    sensors::WriteablePwmSensor,
    units::{Pwm, PwmEnable},
    Hwmons,
};

fn main() {
    let hwmons = Hwmons::parse().unwrap();
    for (_, _, hwmon) in &hwmons {
        for (_, pwm) in hwmon.writeable_pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
        }
    }
}
