use serde::Deserialize;
use std::error::Error;
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    time: String,
    temperature_2m: f64,
    relative_humidity_2m: u8,
    wind_speed_10m: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let latitude = 28.6692;
    let longitude = 77.4538;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,wind_speed_10m",
        latitude, longitude
    );

    let response = reqwest::get(&url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    println!("Current Weather (as of {}):", response.current.time);
    println!("Temperature: {}°C", response.current.temperature_2m);
    println!("Humidity: {}%", response.current.relative_humidity_2m);
    println!("Wind Speed: {} km/h", response.current.wind_speed_10m);

    Ok(())
}
