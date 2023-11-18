use serde_json::Value;
use std::collections::VecDeque;
use csv::Writer;
use std::time::Instant;
use crate::api_clients::binance;
use crate::api_clients::okx;
use crate::utils;

// Analyzes historic data from Binance and OKX for arbitrage opportunities.
pub fn historic_arbitrage(binance_data: &Value, okx_data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    // Trading fees for Binance and OKX.
    let binance_fees: f64 = 0.001; // Binance typically charges 0.1% fee.
    let okx_fees: f64 = 0.001; // OKX charges around 0.1% fee.

    // Store found arbitrage opportunities for further analysis.
    let mut arbitrage_opportunities = VecDeque::new();

    // CSV writer to output historical arbitrage opportunities.
    let mut wtr = Writer::from_path("historic_arbitrage.csv")?;
    wtr.write_record(&["LDO OKX Price in UDDT", "OKX Timestamp (ms)", "LDO Binance Price in USDT", "Binance Timestamp (ms)", "Time Difference (ms)", "Price Difference (USDT)", "Net Arbitrage Profit (USDT)"])?;

    // Loop through both Binance and OKX historical trade data to find arbitrage opportunities.
    if let Some(binance_trades) = binance_data.as_array() {
        if let Some(okx_trades) = okx_data.as_array() {
            for o_trade in okx_trades {
                if let Ok(o_time) = utils::parse_timestamp(o_trade, "ts") {
                    // Extract price and time for OKX trade, and find the closest Binance trade.
                    let o_price: f64 = o_trade["px"].as_str().ok_or("Price field missing for OKX trade")?.parse()?;
                    if let Some(closest_b_trade_timestamp) = binance_trades.iter().filter_map(|b_trade| utils::parse_timestamp(b_trade, "time").ok()).min_by_key(|&b_time| (o_time - b_time).abs()) {
                        if let Some(closest_b_trade) = binance_trades.iter().find(|&b_trade| match utils::parse_timestamp(b_trade, "time") {
                            Ok(b_time) => b_time == closest_b_trade_timestamp,
                            Err(_) => false,
                        }) {
                            // Calculation and checks for potential arbitrage opportunity.
                            let b_time: i64 = utils::parse_timestamp(closest_b_trade, "time").unwrap_or(i64::MAX);
                            let time_diff = (o_time - b_time).abs();
                            let b_price: f64 = closest_b_trade["price"].as_str().unwrap().parse()?;
                            if time_diff <= 5 {
                                let price_diff = (b_price - o_price).abs();
                                let total_fees = (b_price * binance_fees / 100.0) + (o_price * okx_fees / 100.0);
                                if price_diff > total_fees {
                                    // Store and write arbitrage opportunity to CSV.
                                    arbitrage_opportunities.push_back((closest_b_trade["time"].to_string(), o_trade["ts"].to_string(), price_diff));
                                    let real_price_diff = price_diff - total_fees;
                                    wtr.write_record(&[o_price.to_string(), o_time.to_string(), b_price.to_string(), b_time.to_string(), time_diff.to_string(), price_diff.to_string(), real_price_diff.to_string()])?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Number of Arbitrage Opportunities: {}", arbitrage_opportunities.len());
    Ok(())
}

// Function for monitoring and identifying live arbitrage opportunities.
pub async fn live_arbitrage(symbol: &str) {
    // Setting rate limits and fees for the API requests to Binance and OKX.
    let binance_rate_limit = 1;
    let okx_rate_limit = 1;
    let binance_fees: f64 = 0.001; // Fee rate for Binance.
    let okx_fees: f64 = 0.001; // Fee rate for OKX.

    // Infinite loop for continuous monitoring.
    loop {
        let start_time = Instant::now();

        // Fetching the latest trade data from Binance and OKX.
        let binance_data = binance::get_binance_data(symbol, binance_rate_limit).await.unwrap();
        let okx_data = okx::get_okx_data(symbol, okx_rate_limit).await.unwrap();

        // Measure the duration of the API calls.
        let api_call_duration = start_time.elapsed();

        // Extracting the latest trade details from Binance and OKX data.
        let binance_latest_trade = binance_data.as_array().unwrap().first().unwrap();
        let okx_latest_trade = okx_data.as_array().unwrap().first().unwrap();

        // Parsing price and timestamp information from the trades.
        let b_price: f64 = binance_latest_trade["price"].as_str().unwrap().parse().unwrap();
        let o_price: f64 = okx_latest_trade["px"].as_str().unwrap().parse().unwrap();
        let b_time: i64 = utils::parse_timestamp(binance_latest_trade, "time").unwrap();
        let o_time: i64 = utils::parse_timestamp(okx_latest_trade, "ts").unwrap();

        // Start timing the logic execution.
        let logic_execution_start_time = Instant::now();

        // Identifying arbitrage opportunity by comparing prices and accounting for fees.
        if (b_price - o_price).abs() > (b_price * binance_fees / 100.0 + o_price * okx_fees / 100.0) {
            println!("\n\n");
            // Determine which exchange to buy from and sell to.
            let (buy_exchange, buy_price, sell_exchange, sell_price) = if b_price < o_price {
                ("Binance", b_price, "OKX", o_price)
            } else {
                ("OKX", o_price, "Binance", b_price)
            };
            // Calculate the net profit after accounting for fees.
            let net_profit = (sell_price - buy_price) - (sell_price * binance_fees / 100.0 + buy_price * okx_fees / 100.0);

            // Calculate the duration of the logic execution.
            let logic_execution_duration = logic_execution_start_time.elapsed();
            
            // Print the arbitrage opportunity details.
            println!("Arbitrage opportunity detected at timestamp {}.\nBuy LDO on {} at {} USDT and sell on {} at {} USDT.\nNet profit: {} USDT.\nAPI Call Duration: {:?}, Logic Execution Duration: {:?}",
                     std::cmp::max(b_time, o_time), buy_exchange, buy_price, sell_exchange, sell_price, net_profit,
                     api_call_duration, logic_execution_duration);
        }

        // Pause for a second before the next iteration.
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
