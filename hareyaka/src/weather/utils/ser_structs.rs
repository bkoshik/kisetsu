use serde::Serialize;
use crate::units::Units;

#[derive(Serialize)]
pub struct CurrentData {
    pub(crate) location: Location,

    pub(crate) current_weather: CurrentWeather,

    pub(crate) ephemeris: Ephemeris,

    pub(crate) units: Units,
    pub(crate) updated_time: String,
}

#[derive(Serialize)]
pub struct Location {
    pub(crate) area: String,
    pub(crate) region: String,
    pub(crate) country: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

#[derive(Serialize)]
pub struct CurrentWeather {
    pub(crate) description: String,
    pub(crate) temperature: Temperature,
    pub(crate) wind: Wind,
    pub(crate) uv_index: i16,
    pub(crate) humidity: i16,
    pub(crate) visibility: i16,
    pub(crate) pressure: i16,
    
    pub(crate) weather_code: i16,
}

#[derive(Serialize)]
pub struct Temperature {
    pub(crate) temp: i16,
    pub(crate) feels_temp: i16,
    pub(crate) max_temp: i16,
    pub(crate) min_temp: i16,
}

#[derive(Serialize)]
pub struct Wind {
    pub(crate) wind_speed: i16,
    pub(crate) win_direction: i16,
}

#[derive(Serialize)]
pub struct Ephemeris {
    pub(crate) sunrise: String,
    pub(crate) sunset: String,
    pub(crate) moonrise: String,
    pub(crate) moonset: String,
    pub(crate) moon_illumination: String,
    pub(crate) moon_phase: String,
}