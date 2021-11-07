use crate::units::{Error as UnitError, Raw, Result as UnitResult};

use std::borrow::Cow;

/// Enum that represents the different temp sensor types.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TempType {
    CpuEmbeddedDiode,
    Transistor,
    ThermalDiode,
    Thermistor,
    AmdAmdsi,
    IntelPeci,
}

impl Raw for TempType {
    fn from_raw(raw: &str) -> UnitResult<Self> {
        match raw {
            "1" => Ok(TempType::CpuEmbeddedDiode),
            "2" => Ok(TempType::Transistor),
            "3" => Ok(TempType::ThermalDiode),
            "4" => Ok(TempType::Thermistor),
            "5" => Ok(TempType::AmdAmdsi),
            "6" => Ok(TempType::IntelPeci),
            _ => Err(UnitError::raw_conversion(raw)),
        }
    }

    fn to_raw(&self) -> Cow<str> {
        match self {
            TempType::CpuEmbeddedDiode => Cow::from("1"),
            TempType::Transistor => Cow::from("2"),
            TempType::ThermalDiode => Cow::from("3"),
            TempType::Thermistor => Cow::from("4"),
            TempType::AmdAmdsi => Cow::from("5"),
            TempType::IntelPeci => Cow::from("6"),
        }
    }
}
