#![ allow(unused)] 

/// This exmaple is based on this documentation: https://medium.com/@enravishjeni411/building-a-simple-weather-app-%EF%B8%8F-in-rust-a-beginners-guide-971a319a83d4
use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct WeatherData {
    main:Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Deserialize)]
struct Weather {
    desc: String,
}

#[tokio::main]
async fn main() {
    // Set your OpenWeatherMap API key here
    let api_key = "6c047ad5ed479de3729d80af11f62098";
    // Get the city name from the user
    let city_clone = String::from("London.uk");
    // Spawn a Tokio task to run the asynchronous fetch_weather function
    tokio::spawn(fetch_weather(api_key, city_clone));
    // Sleep to keep the program running while the asynchronous task completes
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
}

// Async function to fetch weather data
async fn fetch_weather(api_key: &str, city: String) -> Result<(), reqwest::Error> {
    // Build the URL for the OpenWeatherMap API request
    // let url = format!(
    //     "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
    //     city.trim(),
    //     api_key
    // );

    let url = format!("https://api.openweathermap.org/data/2.5/weather?q=London,uk&APPID=baa24a7db0fe9cd4e4e29c78c683b2d6");
// Make the API request and handle errors
    let response = reqwest::get(&url).await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: WeatherData = response.json().await?;
        
        println!("{:?}", weather_data.main.temp);
        // Extract and print relevant weather information
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].desc;
        println!("Weather in {}: {:.2}°C, {}", city.trim(), temperature, description);
    } else {
        // Print an error message if the request was not successful
        println!("Error: {}", response.status());
    }
    Ok(())
}
