[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/onur/cargo-license/master/LICENSE)

# libmedium
A safe rust library to communicate with the sysfs interface of lm-sensors.

## Usage

Just add this to your `Cargo.toml` file:

```
[dependencies]
libmedium = "0.2"
```

## Cargo-Features

### Standard features

* `writable`: Standard feature that enables all functions that write to sysfs. This includes setting pwm values or disabling sensors.

### Non standard features

* `measurements_units`: Sensor values are returned in types from the [`measurements`](https://crates.io/crates/measurements) crate.
* `unrestricted_parsing`: This feature allows parsing of paths other than '/sys/class/hwmon'. This should only be useful for testing and debugging.

## Examples

* Print the temperature of all the temp sensors in your system:

```rust
use libmedium::{
    Hwmon, Hwmons,
    sensors::{Sensor, SensorBase},
};

let hwmons = Hwmons::parse_read_only().unwrap();
for (hwmon_index, hwmon_name, hwmon) in &hwmons {
    println!("hwmon{} with name {}:", hwmon_index, hwmon_name);
    for (_, temp_sensor) in hwmon.temps() {
        let temperature = temp_sensor.read_input().unwrap();
        println!("\t{}: {}", temp_sensor.name(), temperature);
    }
}
```

* Set the pwm value of all your pwm capable fans to full speed (this requires the `writable` feature to not be disabled):

```rust
use libmedium::{
    Hwmon, Hwmons,
    sensors::PwmSensor,
    units::{Pwm, PwmEnable},
};

let hwmons = Hwmons::parse_read_write().unwrap();
for (_, _, hwmon) in &hwmons {
    for (_, pwm) in hwmon.pwms() {
        pwm.write_enable(PwmEnable::ManualControl).unwrap();
        pwm.write_pwm(Pwm::from_percent(100.0)).unwrap();
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
