use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;

use uom::si::electric_current::milliampere as MilliAmps;

/// Type alias for `uom::si::electric_current::ElectricCurrent<uom::si::SI<f64>, f64>`.
pub type Current = uom::si::electric_current::ElectricCurrent<uom::si::SI<f64>, f64>;

impl Raw for Current {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Current::new::<MilliAmps>)
            .map_err(|_| UnitError::raw_conversion(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<MilliAmps>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::MilliAmps;
    use crate::units::{Current, Raw};

    #[test]
    fn test_from_raw() {
        let av = Current::from_raw("200").unwrap();
        assert_eq!(av.get::<MilliAmps>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Current::new::<MilliAmps>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Current::new::<MilliAmps>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Current::new::<MilliAmps>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
