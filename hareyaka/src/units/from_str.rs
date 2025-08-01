use std::io::{Error, ErrorKind};
use std::str::FromStr;
use crate::units::Units;

impl FromStr for Units {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "metric" => Ok(Units::Metric),
            "imperial" => Ok(Units::Imperial),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("invalid unit: {}", s))),
        }
    }
}