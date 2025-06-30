use std::sync::Arc;
use parking_lot::RwLock;
use warp::Filter;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Metrics {
    pub imbalance: f64,
    pub bid: f64,
    pub ask: f64,
    pub inventory: f64,
}

pub async fn run_dashboard(metrics: Arc<RwLock<Metrics>>) {
    let metrics_filter = warp::any().map(move || metrics.clone());
    let metrics_route = warp::path("metrics")
        .and(metrics_filter)
        .map(|metrics: Arc<RwLock<Metrics>>| {
            warp::reply::json(&*metrics.read())
        });
    warp::serve(metrics_route).run(([127, 0, 0, 1], 8080)).await;
}
