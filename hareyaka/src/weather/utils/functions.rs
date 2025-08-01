use std::io::Error;
use crate::weather::utils::deser_raw_structs::ValueText;

pub fn parse_to_int(s: &str) -> Option<i16> {
    return s.parse::<i16>().ok()
}

pub fn parse_to_float(s: &str) -> Option<f64> {
    return s.parse::<f64>().ok()
}

pub fn get_value(v: &Vec<ValueText>, err: fn() -> Error) -> Result<String, Error> {
    return Ok(v.first().ok_or_else(err)?.value.clone())
}