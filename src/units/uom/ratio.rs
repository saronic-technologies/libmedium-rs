use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::ratio::percent as Percent;

/// Type alias for `uom::si::ratio::Ratio<uom::si::SI<f64>, f64>`.
pub type Ratio = uom::si::ratio::Ratio<uom::si::SI<f64>, f64>;

impl Raw for Ratio {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Ratio::new::<Percent>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<Percent>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::Percent;
    use crate::units::{Ratio, Raw};

    #[test]
    fn test_from_raw() {
        let av = Ratio::from_raw("200").unwrap();
        assert_eq!(av.get::<Percent>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Ratio::new::<Percent>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Ratio::new::<Percent>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Ratio::new::<Percent>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
