use std::env;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use cryptictrader::{
    binance,
    dashboard::{self, Metrics},
    market_maker::{MarketMaker, Side},
    order_manager::OrderManager,
    orderbook::OrderBook,
    risk::RiskManager,
};
use parking_lot::{Mutex, RwLock};

fn handle_update(
    mm: &Mutex<MarketMaker>,
    risk: &Mutex<RiskManager>,
    metrics: &RwLock<Metrics>,
    ob: &OrderBook,
    mm_enabled: &AtomicBool,
) {
    if let (Some((bid, _)), Some((ask, _))) = (ob.best_bid(), ob.best_ask()) {
        let imbalance = ob.imbalance().unwrap_or(0.0);
        let (qbid, qask) = mm.lock().quote(bid, ask, imbalance);
        risk.lock().mark_price((bid + ask) / 2.0);

        if mm_enabled.load(Ordering::Relaxed) {
            OrderManager::submit(Side::Buy, qbid, 0.1);
            OrderManager::submit(Side::Sell, qask, 0.1);
            risk.lock().update_trade(0.1, qbid);
            risk.lock().update_trade(-0.1, qask);
        }

        let mut m = metrics.write();
        m.imbalance = imbalance;
        m.bid = qbid;
        m.ask = qask;
        m.inventory = mm.lock().inventory();
        m.pnl = risk.lock().pnl();
        m.risk_triggered = risk.lock().triggered.load(Ordering::Relaxed);
        m.mm_enabled = mm_enabled.load(Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut args = env::args().skip(1);
    let metrics = Arc::new(RwLock::new(Metrics::default()));
    let mm_enabled = Arc::new(AtomicBool::new(true));
    let dashboard_metrics = metrics.clone();
    let dashboard_flag = mm_enabled.clone();
    tokio::spawn(async move { dashboard::run_dashboard(dashboard_metrics, dashboard_flag).await });

    let mm = Arc::new(Mutex::new(MarketMaker::new(5.0, 0.5)));
    let risk = Arc::new(Mutex::new(RiskManager::new(2.0, 10.0)));

    match args.next().as_deref() {
        Some("--file") => {
            if let Some(path) = args.next() {
                let mm_cl = mm.clone();
                let metrics_cl = metrics.clone();
                let risk_cl = risk.clone();
                let flag = mm_enabled.clone();
                if let Err(e) = binance::stream_depth_offline(&path, move |ob| {
                    handle_update(&mm_cl, &risk_cl, &metrics_cl, ob, &flag);
                })
                .await
                {
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
            let risk_cl = risk.clone();
            let flag = mm_enabled.clone();
            if let Err(e) = binance::stream_depth(sym, move |ob| {
                handle_update(&mm_cl, &risk_cl, &metrics_cl, ob, &flag);
            })
            .await
            {
                eprintln!("{}", e);
            }
        }
    }
}
