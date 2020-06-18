use libmedium::{
    sensors::PwmSensor,
    units::{Pwm, PwmEnable},
    Hwmon, Hwmons,
};

fn main() {
    let hwmons = Hwmons::parse_read_write().unwrap();
    for (_, _, hwmon) in &hwmons {
        for (_, pwm) in hwmon.pwms() {
            pwm.write_enable(PwmEnable::ManualControl).unwrap();
            pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
        }
    }
}
