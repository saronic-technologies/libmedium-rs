use crate::units::{Error as RawError, Raw, Result as RawSensorResult};

use std::borrow::Cow;

use uom::si::frequency::hertz as Hertz;

/// Type alias for `uom::si::frequency::Frequency<uom::si::SI<f64>, f64>`.
pub type Frequency = uom::si::frequency::Frequency<uom::si::SI<f64>, f64>;

impl Raw for Frequency {
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(Frequency::new::<Hertz>)
            .map_err(|_| RawError::from(raw))
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<Hertz>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::Hertz;
    use crate::units::{Frequency, Raw};

    #[test]
    fn test_from_raw() {
        let av = Frequency::from_raw("200").unwrap();
        assert_eq!(av.get::<Hertz>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = Frequency::new::<Hertz>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Frequency::new::<Hertz>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = Frequency::new::<Hertz>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
