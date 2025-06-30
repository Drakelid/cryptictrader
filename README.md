# CrypticTrader

Experimental high-frequency trading system in Rust.

The engine computes order book imbalance and drives a very simple market maker
that skews quotes based on that signal. Live metrics are exposed via a local
web dashboard at `http://localhost:8080/metrics`.

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
