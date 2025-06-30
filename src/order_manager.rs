use crate::market_maker::Side;
use log::info;

pub struct OrderManager;

impl OrderManager {
    pub fn submit(side: Side, price: f64, qty: f64) {
        info!("Order {:?} {} @ {}", side, qty, price);
    }
}
