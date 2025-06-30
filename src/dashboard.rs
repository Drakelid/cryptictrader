use parking_lot::RwLock;
use serde::Serialize;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use warp::Filter;

#[derive(Default, Serialize)]
pub struct Metrics {
    pub imbalance: f64,
    pub bid: f64,
    pub ask: f64,
    pub inventory: f64,
    pub pnl: f64,
    pub risk_triggered: bool,
    pub mm_enabled: bool,
}

pub async fn run_dashboard(metrics: Arc<RwLock<Metrics>>, mm_enabled: Arc<AtomicBool>) {
    let metrics_filter = warp::any().map(move || metrics.clone());
    let metrics_route = warp::path("metrics")
        .and(metrics_filter)
        .map(|metrics: Arc<RwLock<Metrics>>| warp::reply::json(&*metrics.read()));

    let mm_state = mm_enabled.clone();
    let toggle_route = warp::path("toggle_mm")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(move |params: std::collections::HashMap<String, String>| {
            if let Some(v) = params.get("enabled") {
                let enable = v == "1" || v.to_lowercase() == "true";
                mm_state.store(enable, Ordering::Relaxed);
            }
            "ok"
        });

    let routes = metrics_route.or(toggle_route);
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
