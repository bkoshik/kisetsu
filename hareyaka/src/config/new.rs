use crate::units::Units;
use crate::config::Config;

impl Config {
    pub fn new(loc: String, units: Units) -> Self {
        return Self {
            location: loc,
            units: units
        }
    }
}