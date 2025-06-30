# CrypticTrader

Experimental high-frequency trading system in Rust. It streams live order book
data, computes imbalance signals and places skewed quotes via a tiny market
maker. A simple risk manager tracks P&L and position limits.

The engine computes order book imbalance and drives a very simple market maker
that skews quotes based on that signal. Live metrics are exposed via a local
web dashboard at `http://localhost:8080/metrics`. The dashboard also exposes
`/toggle_mm?enabled=0` to disable or enable quoting on the fly.

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
