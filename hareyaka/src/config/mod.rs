use crate::units::Units;

pub mod new;

pub struct Config {
    pub(crate) location: String,
    pub(crate) units: Units,
}