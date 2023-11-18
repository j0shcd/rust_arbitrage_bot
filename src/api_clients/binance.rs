use reqwest;
use serde_json::Value;
use std::error::Error;

// Public function to fetch trade data from Binance's API.
pub async fn get_binance_data(symbol: &str, limit: usize) -> Result<Value, Box<dyn Error>> {
    // Construct the URL for Binance's trades endpoint with the given symbol and limit.
    let url = format!(
        "https://api.binance.com/api/v3/trades?symbol={}&limit={}",
        symbol, limit
    );

    // Perform an asynchronous GET request to the Binance API.
    let resp = reqwest::get(&url).await?;

    if resp.status().is_success() {
        // Parse the response body as JSON.
        let data: Value = resp.json().await?;
        // Return the parsed data.
        Ok(data)
    } else {
        // In case of an unsuccessful request, return the error.
        Err(Box::new(resp.error_for_status().err().unwrap()))
    }
}
