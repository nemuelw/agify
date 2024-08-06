use anyhow::{anyhow, Error};
use serde_derive::Deserialize;

const API_URL: &str = "https://api.agify.io";

#[derive(Debug, Deserialize)]
pub struct AgeEstimation {
    pub count: u32,
    pub name: String,
    pub age: Option<u8>
}

#[derive(Debug, Deserialize)]
pub struct AgifyError {
    pub error: String
}

pub async fn estimate_age(name: &str) -> Result<AgeEstimation, Error> {
    let url = format!("{}?name={}", API_URL, name);
    let response = reqwest::get(url).await.map_err(|e| {
        anyhow!("{}", e)
    })?;
    if response.status().is_success() {
        let age_estimation: AgeEstimation = response.json().await?;
        Ok(age_estimation)
    } else {
        let api_error: Result<AgifyError, _> = response.json::<AgifyError>().await;
        match api_error {
            Ok(api_error) => Err(anyhow!("{}", api_error.error)),
            Err(e) => Err(anyhow!("{}", e))
        }
    }
}
