# CrypticTrader

Experimental high-frequency trading system in Rust.

This prototype connects to Binance's depth WebSocket stream and prints a running
order book imbalance. Run with:

```bash
cargo run --release -- btcusdt
```

If network access is unavailable the program will exit after printing an error.
