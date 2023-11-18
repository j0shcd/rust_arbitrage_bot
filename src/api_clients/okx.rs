use reqwest;
use serde_json::Value;
use std::error::Error;
use crate::utils::split_symbol_okx;

// Asynchronous function to fetch trade data from OKX exchange.
pub async fn get_okx_data(imported_symbol: &str, limit: usize) -> Result<Value, Box<dyn Error>> {
    // Splitting the symbol into a format accepted by OKX API.
    let split_index = 3;
    let symbol = split_symbol_okx(imported_symbol, split_index);

    // Creating the URL for the OKX trades endpoint with the modified symbol and limit.
    let url = format!(
        "https://www.okx.com/api/v5/market/trades?instId={}&limit={}",
        symbol, limit
    );

    // Performing an asynchronous GET request to the OKX API.
    let resp = reqwest::get(&url).await?;

    if !resp.status().is_success() {
        // In case of an unsuccessful request, return the error.
        return Err(Box::new(resp.error_for_status().err().unwrap()));
    }

    // Parsing the response body as JSON.
    let body: serde_json::Value = resp.json().await?;

    // Checking if the API response is successful (code "0" indicates success).
    if body["code"] == "0" {
        // Returning the data part of the response.
        Ok(body["data"].clone())
    } else {
        // Logging and returning an error for non-success codes.
        println!("The error code is: {}", body["code"]);
        Err("Error".into())
    }
}
