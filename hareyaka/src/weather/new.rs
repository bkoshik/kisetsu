use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use reqwest::blocking::Client;
use crate::config::Config;
use crate::units::Units;
use crate::weather::{
    Hareyaka,
    utils::{
        functions::*,
        ser_structs::*,
        deser_raw_structs::WeatherData,
    }
};

impl Hareyaka {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let client = Client::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("format", "j2");

        let url: String = format!("https://wttr.in/{}",
                                  config.location);

        let response = match client.get(&url).query(&params).send() {
            Ok(response) => response,
            Err(error) => return Err(Error::new(ErrorKind::Other, error))
        };

        if let Ok(data) = &response.json::<WeatherData>() {
            let err = || Error::new(ErrorKind::InvalidData, "Invalid data");

            let current_condition =
                data.current_condition.first().ok_or_else(err)?;
            let nearest_area =
                data.nearest_area.first().ok_or_else(err)?;
            let weather_days =
                data.weather_days.first().ok_or_else(err)?;
            let astronomy =
                weather_days.astronomy.first().ok_or_else(err)?;

            let (area, region, country, latitude, longitude) = (
                get_value(&nearest_area.area_name, err)?,
                get_value(&nearest_area.region, err)?,
                get_value(&nearest_area.country, err)?,
                parse_to_float(&nearest_area.latitude).ok_or_else(err)?,
                parse_to_float(&nearest_area.longitude).ok_or_else(err)?,
            );
            let description = current_condition.weather_desc.first().ok_or_else(err)?.value.clone();
            let (temp, feels_temp, max_temp, min_temp) =
                match config.units {
                    Units::Metric => (
                        parse_to_int(&current_condition.temp_c).ok_or_else(err)?,
                        parse_to_int(&current_condition.feels_like_c).ok_or_else(err)?,
                        parse_to_int(&weather_days.maxtemp_c).ok_or_else(err)?,
                        parse_to_int(&weather_days.mintemp_c).ok_or_else(err)?
                    ),
                    Units::Imperial => (
                        parse_to_int(&current_condition.temp_f).ok_or_else(err)?,
                        parse_to_int(&current_condition.feels_like_f).ok_or_else(err)?,
                        parse_to_int(&weather_days.maxtemp_f).ok_or_else(err)?,
                        parse_to_int(&weather_days.mintemp_f).ok_or_else(err)?
                    )
                };
            let (wind_speed, wind_direction) =
                match config.units {
                    Units::Metric => (
                        parse_to_int(&current_condition.windspeed_kmph).ok_or_else(err)?,
                        parse_to_int(&current_condition.winddir_degree).ok_or_else(err)?
                    ),
                    Units::Imperial => (
                        parse_to_int(&current_condition.windspeed_miles).ok_or_else(err)?,
                        parse_to_int(&current_condition.winddir_degree).ok_or_else(err)?
                    )
                };
            let uv_index = parse_to_int(&current_condition.uv_index).ok_or_else(err)?;
            let humidity = parse_to_int(&current_condition.humidity).ok_or_else(err)?;
            let visibility = parse_to_int(&current_condition.visibility).ok_or_else(err)?;
            let pressure = parse_to_int(&current_condition.pressure).ok_or_else(err)?;

            let (sunrise, sunset, moonrise, moonset, moon_illumination, moon_phase) = (
                astronomy.sunrise.clone(),
                astronomy.sunset.clone(),
                astronomy.moonrise.clone(),
                astronomy.moonset.clone(),
                astronomy.moon_illumination.clone(),
                astronomy.moon_phase.clone(),
            );

            let weather_code = parse_to_int(&current_condition.weather_code).ok_or_else(err)?;
            let updated_time = current_condition.local_obs_date_time.clone();

            return Ok(Self {
                data: CurrentData {
                    location: Location {
                        area: area,
                        region: region,
                        country: country,
                        latitude: latitude,
                        longitude: longitude,
                    },
                    current_weather: CurrentWeather {
                        description: description,
                        temperature: Temperature {
                            temp: temp,
                            feels_temp: feels_temp,
                            max_temp: max_temp,
                            min_temp: min_temp,
                        },
                        wind: Wind {
                            wind_speed: wind_speed,
                            win_direction: wind_direction
                        },
                        uv_index: uv_index,
                        humidity: humidity,
                        visibility: visibility,
                        pressure: pressure,

                        weather_code: weather_code,
                    },

                    ephemeris: Ephemeris {
                        sunrise: sunrise,
                        sunset: sunset,
                        moonrise: moonrise,
                        moonset: moonset,
                        moon_illumination: moon_illumination,
                        moon_phase: moon_phase,
                    },
                    
                    units: config.units.clone(),
                    updated_time: updated_time,
                }
            })
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid data"))
        }
    }
}