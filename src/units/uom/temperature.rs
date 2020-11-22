use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::thermodynamic_temperature::millikelvin as MilliKelvin;

/// Type alias for `uom::si::thermodynamic_temperature::ThermodynamicTemperature<uom::si::SI<f64>, f64>`.
pub type Temperature =
    uom::si::thermodynamic_temperature::ThermodynamicTemperature<uom::si::SI<f64>, f64>;

impl Raw for Temperature {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(|celsius| celsius + 273150.0)
            .map(Temperature::new::<MilliKelvin>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!(
            "{:.0}",
            self.get::<MilliKelvin>().round() - 273150.0
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::units::{Raw, Temperature};
    use uom::si::thermodynamic_temperature::degree_celsius as Celsius;

    #[test]
    fn test_from_raw() {
        let av = Temperature::from_raw("60000").unwrap();
        assert_eq!(av.get::<Celsius>().round(), 60.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Temperature::new::<Celsius>(60.0);
        assert_eq!(av.to_raw().as_ref(), "60000");

        let av = Temperature::new::<Celsius>(60.25);
        assert_eq!(av.to_raw().as_ref(), "60250");

        let av = Temperature::new::<Celsius>(59.7);
        assert_eq!(av.to_raw().as_ref(), "59700");
    }
}
