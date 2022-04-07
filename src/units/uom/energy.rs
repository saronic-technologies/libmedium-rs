use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;

use uom::si::energy::microjoule as MicroJoules;

/// Type alias for `uom::si::energy::Energy<uom::si::SI<f64>, f64>`.
pub type Energy = uom::si::energy::Energy<uom::si::SI<f64>, f64>;

impl Raw for Energy {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Energy::new::<MicroJoules>)
            .map_err(|_| UnitError::raw_conversion(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<MicroJoules>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::MicroJoules;
    use crate::units::{Energy, Raw};

    #[test]
    fn test_from_raw() {
        let av = Energy::from_raw("200").unwrap();
        assert_eq!(av.get::<MicroJoules>().round(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Energy::new::<MicroJoules>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Energy::new::<MicroJoules>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Energy::new::<MicroJoules>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
