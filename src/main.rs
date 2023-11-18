mod api_clients;
use api_clients::binance;
use api_clients::okx;
mod utils;
mod arbitrage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "LDOUSDT";
    let binance_rate_limit = 1000; // Binance's rate limit for requests.
    let okx_rate_limit = 500; // OKX's rate limit for requests.

    // Fetch market data from Binance and OKX exchanges.
    let binance_data = binance::get_binance_data(symbol, binance_rate_limit).await?;
    let okx_data = okx::get_okx_data(symbol, okx_rate_limit).await?;

    // Analyze historic arbitrage opportunities between the two data sets.
    arbitrage::historic_arbitrage(&binance_data, &okx_data)?;
    println!("You can see historic arbitrage opportunity details in historic_arbitrage.csv");

    // Monitor and analyze live arbitrage opportunities.
    println!("________________________________________________________________________________");
    println!("Starting live arbitrage monitoring...\n");

    arbitrage::live_arbitrage(symbol).await;    
    Ok(())
}
