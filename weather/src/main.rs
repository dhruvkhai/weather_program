use reqwest::Client;
use serde_json::Value;
use std::error::Error;

// Entry point of the application with async runtime using tokio.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Weather Program");
    println!("----------------");

    loop {
        println!("1. Get Current Weather");
        println!("2. Get Forecast");
        println!("3. Exit");

        let mut user_choice = String::new();
        std::io::stdin().read_line(&mut user_choice).expect("Failed to read input");

        let user_choice = user_choice.trim().parse::<u8>().expect("Invalid choice");

        match user_choice {
            1 => get_current_weather().await?,
            2 => get_forecast().await?,
            3 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
    Ok(())
}

// Asynchronously fetch and display the current weather information.
async fn get_current_weather() -> Result<(), Box<dyn Error>> {
    let api_key: &str = "e202d8cb2f6a2f619a838cd85d4d48d0"; // Replace with your actual API key
    println!("Enter city/country name or zip code:");

    let mut location = String::new();
    std::io::stdin().read_line(&mut location).expect("Failed to read input");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        location.trim(),
        api_key
    );

    let weather_data = get_weather_data(&url).await?;

    println!("Weather in {}:", weather_data["name"]);
    println!("  Temperature: {:.2}°C", weather_data["main"]["temp"].as_f64().unwrap());
    println!("  Humidity: {}%", weather_data["main"]["humidity"]);
    println!("  Wind Speed: {:.2} m/s", weather_data["wind"]["speed"].as_f64().unwrap());
    println!("  Description: {}", weather_data["weather"][0]["description"]);
    Ok(())
}

// Asynchronously fetch and display the weather forecast.
async fn get_forecast() -> Result<(), Box<dyn Error>> {
    let api_key: &str = "e202d8cb2f6a2f619a838cd85d4d48d0"; // Replace with your actual API key
    println!("Enter city/country name or zip code:");

    let mut location = String::new();
    std::io::stdin().read_line(&mut location).expect("Failed to read input");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={}&units=metric&appid={}",
        location.trim(),
        api_key
    );

    let forecast_data = get_weather_data(&url).await?;

    println!("Forecast:");
    for forecast in forecast_data["list"].as_array().expect("Invalid forecast data") {
        println!("  Date: {}", forecast["dt_txt"]);
        println!("    Temperature: {:.2}°C", forecast["main"]["temp"].as_f64().unwrap());
        println!("    Humidity: {}%", forecast["main"]["humidity"]);
        println!("    Wind Speed: {:.2} m/s", forecast["wind"]["speed"].as_f64().unwrap());
        println!("    Description: {}", forecast["weather"][0]["description"]);
        println!("");
    }
    Ok(())
}

// Asynchronously sends a request to the provided URL and returns JSON data.
async fn get_weather_data(url: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();

    // Asynchronously send the request and await the response.
    let response = client.get(url).send().await?;

    // Check if the response status is successful.
    if response.status().is_success() {
        // Asynchronously read the response text and parse it as JSON.
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        Ok(json)
    } else {
        // Create a custom error with a meaningful message.
        let status_code = response.status();
        let error_message = format!("Error: {} - Unable to get weather data", status_code);
        Err(Box::from(error_message))
    }
}
