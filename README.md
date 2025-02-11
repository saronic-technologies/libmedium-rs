[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/onur/cargo-license/master/LICENSE)

# libmedium
A safe rust library to communicate with the sysfs interface of lm-sensors.

## Usage

Just add this to your `Cargo.toml` file:

```toml
[dependencies]
libmedium = "0.8"
```

## Cargo-Features

### Standard features

* `writeable`: Standard feature that enables all functions that write to sysfs. This includes setting pwm values and disabling sensors.
* `sync`: Build synchronous versions of all sensors.
+ `virtual_sensors`: Feature that lets you create virtual sensors. Virtual sensors don't belong to sysfs but can be any file provided by a driver or the user.

### Non standard features

* `uom_units`: Sensor values are returned as types from the [`uom`](https://crates.io/crates/uom) crate.
* `unrestricted_parsing`: This feature allows parsing of paths other than '/sys/class/hwmon'. This should only be useful for testing and debugging.
* `async`: Build asynchronous versions of all sensors.

## Examples

* Print the temperature of all the temp sensors in your system:

```rust
use libmedium::{
    parse_hwmons,
    sensors::sync_sensors::{temp::TempSensor, Sensor},
};

let hwmons = parse_hwmons().unwrap();
for hwmon in &hwmons {
    println!("hwmon{} with name {}:", hwmon.index(), hwmon.name());
    for (_, temp_sensor) in hwmon.temps() {
        let temperature = temp_sensor.read_input().unwrap();
        println!("\t{}: {}", temp_sensor.name(), temperature);
    }
}
```

* Set the pwm value of all your pwm capable fans to full speed (this requires the `writeable` feature to not be disabled):

```rust
use libmedium::{
    parse_hwmons,
    sensors::sync_sensors::pwm::WriteablePwmSensor,
    units::{Pwm, PwmEnable},
};

let hwmons = parse_hwmons().unwrap();
for hwmon in &hwmons {
    for (_, pwm) in hwmon.writeable_pwms() {
        pwm.write_enable(PwmEnable::ManualControl).unwrap();
        pwm.write_pwm(Pwm::FULLSPEED).unwrap();
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
