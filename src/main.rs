use std::env;
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use cryptictrader::{binance, dashboard::{self, Metrics}, market_maker::MarketMaker, orderbook::OrderBook};

fn handle_update(mm: &Mutex<MarketMaker>, metrics: &RwLock<Metrics>, ob: &OrderBook) {
    if let (Some((bid,_)), Some((ask,_))) = (ob.best_bid(), ob.best_ask()) {
        let imbalance = ob.imbalance().unwrap_or(0.0);
        let (qbid, qask) = mm.lock().quote(bid, ask, imbalance);
        println!("imbalance {:.4} quotes {:.2}/{:.2}", imbalance, qbid, qask);
        let mut m = metrics.write();
        m.imbalance = imbalance;
        m.bid = qbid;
        m.ask = qask;
        m.inventory = mm.lock().inventory();
    }
}

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);
    let metrics = Arc::new(RwLock::new(Metrics::default()));
    let dashboard_metrics = metrics.clone();
    tokio::spawn(async move { dashboard::run_dashboard(dashboard_metrics).await });

    let mm = Arc::new(Mutex::new(MarketMaker::new(5.0, 0.5)));

    match args.next().as_deref() {
        Some("--file") => {
            if let Some(path) = args.next() {
                let mm_cl = mm.clone();
                let metrics_cl = metrics.clone();
                if let Err(e) = binance::stream_depth_offline(&path, move |ob| {
                    handle_update(&mm_cl, &metrics_cl, ob);
                }).await {
                    eprintln!("{}", e);
                }
            } else {
                eprintln!("--file requires a path");
            }
        }
        symbol => {
            let sym = symbol.unwrap_or("btcusdt");
            let mm_cl = mm.clone();
            let metrics_cl = metrics.clone();
            if let Err(e) = binance::stream_depth(sym, move |ob| {
                handle_update(&mm_cl, &metrics_cl, ob);
            }).await {
                eprintln!("{}", e);
            }
        }
    }
}
