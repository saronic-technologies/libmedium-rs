//! Module containing the subfunction traits.

mod alarm;
mod average;
mod beep;
mod crit;
mod enable;
mod faulty;
mod highest;
mod input;
mod low_crit;
mod lowest;
mod max;
mod min;

pub use alarm::*;
pub use average::*;
pub use beep::*;
pub use crit::*;
pub use enable::*;
pub use faulty::*;
pub use highest::*;
pub use input::*;
pub use low_crit::*;
pub use lowest::*;
pub use max::*;
pub use min::*;
