use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::power::microwatt as MicroWatt;

/// Type alias for `uom::si::power::Power<uom::si::SI<f64>, f64>`.
pub type Power = uom::si::power::Power<uom::si::SI<f64>, f64>;

impl Raw for Power {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Power::new::<MicroWatt>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<MicroWatt>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::MicroWatt;
    use crate::units::{Power, Raw};

    #[test]
    fn test_from_raw() {
        let av = Power::from_raw("200").unwrap();
        assert_eq!(av.get::<MicroWatt>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Power::new::<MicroWatt>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Power::new::<MicroWatt>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Power::new::<MicroWatt>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
