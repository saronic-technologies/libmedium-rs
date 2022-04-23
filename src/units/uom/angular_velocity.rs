use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;

use uom::si::angular_velocity::revolution_per_minute as RPM;

/// Type alias for `uom::si::angular_velocity::AngularVelocity<uom::si::SI<f64>, f64>`.
pub type AngularVelocity = uom::si::angular_velocity::AngularVelocity<uom::si::SI<f64>, f64>;

impl Raw for AngularVelocity {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        raw.trim()
            .parse::<f64>()
            .map(AngularVelocity::new::<RPM>)
            .map_err(UnitError::parsing_float)
    }

    fn to_raw(&self) -> Cow<str> {
        Cow::Owned(format!("{:.0}", self.get::<RPM>().round()))
    }
}

#[cfg(test)]
mod tests {
    use super::RPM;
    use crate::units::{AngularVelocity, Raw};

    #[test]
    fn test_from_raw() {
        let av = AngularVelocity::from_raw("200").unwrap();
        assert_eq!(av.get::<RPM>(), 200.0);
    }

    #[test]
    fn test_to_raw() {
        let av = AngularVelocity::new::<RPM>(200.0);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = AngularVelocity::new::<RPM>(200.2);
        assert_eq!(av.to_raw().as_ref(), "200");

        let av = AngularVelocity::new::<RPM>(199.7);
        assert_eq!(av.to_raw().as_ref(), "200");
    }
}
