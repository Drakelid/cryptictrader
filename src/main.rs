use std::env;

use cryptictrader::binance;

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("--file") => {
            if let Some(path) = args.next() {
                if let Err(e) = binance::stream_depth_offline(&path).await {
                    eprintln!("{}", e);
                }
            } else {
                eprintln!("--file requires a path");
            }
        }
        symbol => {
            let sym = symbol.unwrap_or("btcusdt");
            if let Err(e) = binance::stream_depth(sym).await {
                eprintln!("{}", e);
            }
        }
    }
}
