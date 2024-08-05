use anyhow::Error;
use serde_derive::Deserialize;

const API_URL: &str = "https://api.agify.io";

#[derive(Debug, Deserialize)]
pub struct AgeEstimation {
    pub count: u32,
    pub name: String,
    pub age: u8
}

pub async fn estimate_age(name: &str) -> Result<AgeEstimation, Error> {
    let url = format!("{}?name={}", API_URL, name);
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let age_estimation: AgeEstimation = response.json().await?;
        Ok(age_estimation)
    } else {
        Err(anyhow::anyhow!("api request failed with status: {}", response.status()))
    }
}
