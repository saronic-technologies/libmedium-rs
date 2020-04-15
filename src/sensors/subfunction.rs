//! Module containing sensor subfunctions.

use std::fmt;

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub enum SensorSubFunctionType {
    Input,
    Fault,
    Label,
    Type,
    Lowest,
    Highest,
    InputLowest,
    InputHighest,
    Average,
    AverageIntervalMax,
    AverageIntervalMin,
    AverageHighest,
    AverageLowest,
    Accuracy,
    CapMin,
    CapMax,
    Enable,
    Max,
    Min,
    MaxHyst,
    MinHyst,
    Crit,
    CritHyst,
    Emergency,
    EmergencyHyst,
    LowCrit,
    LowCritHyst,
    Offset,
    Div,
    Pulses,
    Target,
    AverageInterval,
    AverageMax,
    AverageMin,
    Cap,
    CapHyst,
    ResetHistory,
    Pwm,
    Mode,
    Freq,
    AutoChannelsTemp,
}

impl SensorSubFunctionType {
    pub(crate) fn read_only_list() -> &'static [SensorSubFunctionType] {
        const ARRAY: [SensorSubFunctionType; 16] = [
            SensorSubFunctionType::Input,
            SensorSubFunctionType::Fault,
            SensorSubFunctionType::Label,
            SensorSubFunctionType::Type,
            SensorSubFunctionType::Lowest,
            SensorSubFunctionType::Highest,
            SensorSubFunctionType::InputLowest,
            SensorSubFunctionType::InputHighest,
            SensorSubFunctionType::Average,
            SensorSubFunctionType::AverageIntervalMax,
            SensorSubFunctionType::AverageIntervalMin,
            SensorSubFunctionType::AverageHighest,
            SensorSubFunctionType::AverageLowest,
            SensorSubFunctionType::Accuracy,
            SensorSubFunctionType::CapMin,
            SensorSubFunctionType::CapMax,
        ];
        &ARRAY
    }

    pub(crate) fn read_write_list() -> &'static [SensorSubFunctionType] {
        const ARRAY: [SensorSubFunctionType; 24] = [
            SensorSubFunctionType::Enable,
            SensorSubFunctionType::Max,
            SensorSubFunctionType::Min,
            SensorSubFunctionType::MaxHyst,
            SensorSubFunctionType::MinHyst,
            SensorSubFunctionType::Crit,
            SensorSubFunctionType::CritHyst,
            SensorSubFunctionType::Emergency,
            SensorSubFunctionType::EmergencyHyst,
            SensorSubFunctionType::LowCrit,
            SensorSubFunctionType::LowCritHyst,
            SensorSubFunctionType::Offset,
            SensorSubFunctionType::Div,
            SensorSubFunctionType::Pulses,
            SensorSubFunctionType::Target,
            SensorSubFunctionType::AverageInterval,
            SensorSubFunctionType::AverageMax,
            SensorSubFunctionType::AverageMin,
            SensorSubFunctionType::Cap,
            SensorSubFunctionType::CapHyst,
            SensorSubFunctionType::Pwm,
            SensorSubFunctionType::Mode,
            SensorSubFunctionType::Freq,
            SensorSubFunctionType::AutoChannelsTemp,
        ];
        &ARRAY
    }

    pub(crate) fn write_only_list() -> &'static [SensorSubFunctionType] {
        const ARRAY: [SensorSubFunctionType; 1] = [SensorSubFunctionType::ResetHistory];
        &ARRAY
    }

    pub(crate) fn read_list() -> impl Iterator<Item = Self> {
        Self::read_only_list()
            .iter()
            .chain(Self::read_write_list())
            .copied()
    }

    pub(crate) fn write_list() -> impl Iterator<Item = Self> {
        Self::write_only_list()
            .iter()
            .chain(Self::read_write_list())
            .copied()
    }

    pub(crate) fn to_suffix(self) -> &'static str {
        match self {
            SensorSubFunctionType::Input => "_input",
            SensorSubFunctionType::Fault => "_fault",
            SensorSubFunctionType::Label => "_label",
            SensorSubFunctionType::Type => "_type",
            SensorSubFunctionType::Lowest => "_lowest",
            SensorSubFunctionType::Highest => "_highest",
            SensorSubFunctionType::InputLowest => "_input_lowest",
            SensorSubFunctionType::InputHighest => "_input_highest",
            SensorSubFunctionType::Average => "_average",
            SensorSubFunctionType::AverageIntervalMax => "_average_interval_max",
            SensorSubFunctionType::AverageIntervalMin => "_average_interval_min",
            SensorSubFunctionType::AverageHighest => "_average_highest",
            SensorSubFunctionType::AverageLowest => "_average_lowest",
            SensorSubFunctionType::Accuracy => "_accuracy",
            SensorSubFunctionType::CapMin => "_cap_min",
            SensorSubFunctionType::CapMax => "_cap_max",
            SensorSubFunctionType::Enable => "_enable",
            SensorSubFunctionType::Max => "_max",
            SensorSubFunctionType::Min => "_min",
            SensorSubFunctionType::MaxHyst => "_max_hyst",
            SensorSubFunctionType::MinHyst => "_min_hyst",
            SensorSubFunctionType::Crit => "_crit",
            SensorSubFunctionType::CritHyst => "_crit_hyst",
            SensorSubFunctionType::Emergency => "_emergency",
            SensorSubFunctionType::EmergencyHyst => "_emergency_hyst",
            SensorSubFunctionType::LowCrit => "_lcrit",
            SensorSubFunctionType::LowCritHyst => "_lcrit_hyst",
            SensorSubFunctionType::Offset => "_offset",
            SensorSubFunctionType::Div => "_div",
            SensorSubFunctionType::Pulses => "_pulses",
            SensorSubFunctionType::Target => "_target",
            SensorSubFunctionType::AverageInterval => "_average_interval",
            SensorSubFunctionType::AverageMax => "_average_max",
            SensorSubFunctionType::AverageMin => "_average_min",
            SensorSubFunctionType::Cap => "_cap",
            SensorSubFunctionType::CapHyst => "_cap_hyst",
            SensorSubFunctionType::ResetHistory => "_reset_history",
            SensorSubFunctionType::Pwm => "",
            SensorSubFunctionType::Mode => "_mode",
            SensorSubFunctionType::Freq => "_freq",
            SensorSubFunctionType::AutoChannelsTemp => "_auto_channels_temp",
        }
    }
}

impl Eq for SensorSubFunctionType {}

impl fmt::Display for SensorSubFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
