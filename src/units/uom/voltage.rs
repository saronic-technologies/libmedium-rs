use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::electric_potential::millivolt as MilliVolt;

/// Type alias for `uom::si::electric_potential::ElectricPotential<uom::si::SI<f64>, f64>`.
pub type Voltage = uom::si::electric_potential::ElectricPotential<uom::si::SI<f64>, f64>;

impl Raw for Voltage {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Voltage::new::<MilliVolt>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<MilliVolt>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::MilliVolt;
    use crate::units::{Raw, Voltage};

    #[test]
    fn test_from_raw() {
        let av = Voltage::from_raw("200").unwrap();
        assert_eq!(av.get::<MilliVolt>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Voltage::new::<MilliVolt>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Voltage::new::<MilliVolt>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Voltage::new::<MilliVolt>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
