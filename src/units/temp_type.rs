use crate::units::{Raw, RawError, RawSensorResult};

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
    fn from_raw(raw: &str) -> RawSensorResult<Self> {
        match raw {
            "1" => Ok(TempType::CpuEmbeddedDiode),
            "2" => Ok(TempType::Transistor),
            "3" => Ok(TempType::ThermalDiode),
            "4" => Ok(TempType::Thermistor),
            "5" => Ok(TempType::AmdAmdsi),
            "6" => Ok(TempType::IntelPeci),
            _ => Err(RawError::from(raw)),
        }
    }

    fn to_raw(&self) -> String {
        match self {
            TempType::CpuEmbeddedDiode => String::from("1"),
            TempType::Transistor => String::from("2"),
            TempType::ThermalDiode => String::from("3"),
            TempType::Thermistor => String::from("4"),
            TempType::AmdAmdsi => String::from("5"),
            TempType::IntelPeci => String::from("6"),
        }
    }
}
