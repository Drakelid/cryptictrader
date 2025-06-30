use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct RiskManager {
    pub position_limit: f64,
    pub max_loss: f64,
    position: f64,
    realized_pnl: f64,
    current_price: f64,
    pub triggered: AtomicBool,
}

impl RiskManager {
    pub fn new(position_limit: f64, max_loss: f64) -> Self {
        Self {
            position_limit,
            max_loss,
            position: 0.0,
            realized_pnl: 0.0,
            current_price: 0.0,
            triggered: AtomicBool::new(false),
        }
    }

    pub fn update_trade(&mut self, qty: f64, price: f64) {
        self.position += qty;
        self.realized_pnl -= qty * price;
        self.current_price = price;
        self.check();
    }

    pub fn mark_price(&mut self, price: f64) {
        self.current_price = price;
        self.check();
    }

    pub fn pnl(&self) -> f64 {
        self.realized_pnl + self.position * self.current_price
    }

    fn check(&self) {
        let pnl = self.pnl();
        if pnl < -self.max_loss || self.position.abs() > self.position_limit {
            self.triggered.store(true, Ordering::Relaxed);
        }
    }

    pub fn reset(&mut self) {
        self.position = 0.0;
        self.realized_pnl = 0.0;
        self.triggered.store(false, Ordering::Relaxed);
    }
}
