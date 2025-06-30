use std::env;

mod binance;
mod orderbook;

#[tokio::main]
async fn main() {
    let symbol = env::args().nth(1).unwrap_or_else(|| "btcusdt".to_string());

    if let Err(e) = binance::stream_depth(&symbol).await {
        eprintln!("{}", e);
    }
}
