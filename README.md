# CrypticTrader

Experimental high-frequency trading system in Rust.

This prototype connects to Binance's depth WebSocket stream and prints a running
order book imbalance. Run with:

```bash
cargo run --release -- btcusdt
```

If network access is unavailable the program can replay depth updates from a
file using `--file`:

```bash
cargo run --release -- --file sample_depth.json
```
