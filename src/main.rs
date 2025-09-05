use dotenv::dotenv;
use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    let api_key = env::var("API_KEY").expect("API_KEY not found in .env");
    let city = std::env::args().nth(1).expect("Please provide a city name");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response: WeatherResponse = get(&url)?.json()?;

    println!("{}", response.name);
    println!("Temperature: {}°C", response.main.temp);
    println!("Humidity: {}%", response.main.humidity);
    println!("Condition: {}", response.weather[0].description);

    Ok(())
}
