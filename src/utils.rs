use serde_json::Value;

/// Function to split a trading symbol into two parts and join them with a dash.
/// This is specifically formatted for the OKX exchange's standard representation of a symbol.
/// For example, `split_symbol_okx("LDOUSDT", 3)` will return "LDO-USDT".
pub fn split_symbol_okx(symbol: &str, split_index: usize) -> String {
    // Splitting the symbol at the specified index.
    let (base, quote) = symbol.split_at(split_index);
    // Combining the two parts with a dash in between.
    format!("{}-{}", base, quote)
}

/// Helper function to parse a timestamp from trade data.
/// It attempts to extract the timestamp based on the provided field name and convert it to an i64.
/// It handles different formats (String or Number) and returns an error if parsing fails or if the field is missing.
pub fn parse_timestamp(trade: &Value, field: &str) -> Result<i64, Box<dyn std::error::Error>> {
    match trade.get(field) {
        // If the field value is a string, try to parse it as i64.
        Some(Value::String(ts_str)) => ts_str.parse::<i64>().map_err(|e| e.into()),
        // If the field value is a number, try to convert it to i64.
        Some(Value::Number(num)) => num.as_i64().ok_or_else(|| "Invalid number format".into()),
        // Return an error if the field is missing or not a string/number.
        _ => Err(format!("Timestamp field '{}' missing or not a string/number", field).into()),
    }
}