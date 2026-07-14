use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
use anyhow::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f32,
    pub hourly: Hourly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub precipitation: Vec<f32>,
}

pub fn get_api_response(latitude: &str, longitude: &str) -> Result<APIResponse, Error> {
    let api_url: String = "https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&hourly=temperature_2m,precipitation&forecast_days=1"
        .replace("{latitude}", latitude.as_ref())
        .replace("{longitude}", longitude.as_ref());

    let api_response = reqwest::blocking::get(&api_url)?;
    let serialized_output: APIResponse = serde_json::from_str::<APIResponse>(api_response.text()?.as_str())?;

    Ok(serialized_output)
}