use serde::Deserialize;

#[derive(Deserialize, Debug)]

struct Weather {
    latitute: f64,
    lonitute: f64,
    current_weather: CurrentWeather,
}
#[derive(Deserialize, Debug)]

struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const Url: &str = "http://localhost:8080";
    let response = reqwest::get(Url).await?;
    let weather: serde_json::Value = response.json().await?;
    println!("weather: {:?}", weather);
    Ok(())
}

