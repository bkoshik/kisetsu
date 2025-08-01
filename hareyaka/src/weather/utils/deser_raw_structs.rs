use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub current_condition: Vec<CurrentCondition>,
    pub nearest_area: Vec<NearestArea>,
    pub request: Vec<Request>,

    #[serde(rename = "weather")]
    pub weather_days: Vec<WeatherDay>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentCondition {
    #[serde(rename = "FeelsLikeC")]
    pub feels_like_c: String,
    #[serde(rename = "FeelsLikeF")]
    pub feels_like_f: String,
    pub cloudcover: String,
    pub humidity: String,
    pub local_obs_date_time: String,
    #[serde(rename = "observation_time")]
    pub observation_time: String,
    pub precip_inches: String,
    #[serde(rename = "precipMM")]
    pub precip_mm: String,
    pub pressure: String,
    pub pressure_inches: String,
    #[serde(rename = "temp_C")]
    pub temp_c: String,
    #[serde(rename = "temp_F")]
    pub temp_f: String,
    pub uv_index: String,
    pub visibility: String,
    pub visibility_miles: String,
    pub weather_code: String,
    pub weather_desc: Vec<ValueText>,
    pub weather_icon_url: Vec<ValueText>,
    pub winddir16_point: String,
    pub winddir_degree: String,
    pub windspeed_kmph: String,
    pub windspeed_miles: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NearestArea {
    pub area_name: Vec<ValueText>,
    pub country: Vec<ValueText>,
    pub latitude: String,
    pub longitude: String,
    pub population: String,
    pub region: Vec<ValueText>,
    pub weather_url: Vec<ValueText>,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub query: String,
    #[serde(rename = "type")]
    pub request_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherDay {
    pub astronomy: Vec<Astronomy>,
    pub avgtemp_c: String,
    pub avgtemp_f: String,
    pub date: String,
    pub maxtemp_c: String,
    pub maxtemp_f: String,
    pub mintemp_c: String,
    pub mintemp_f: String,
    pub sun_hour: String,
    #[serde(rename = "totalSnow_cm")]
    pub total_snow_cm: String,
    pub uv_index: String,
}

#[derive(Debug, Deserialize)]
pub struct Astronomy {
    pub moon_illumination: String,
    pub moon_phase: String,
    pub moonrise: String,
    pub moonset: String,
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Debug, Deserialize)]
pub struct ValueText {
    pub value: String,
}