
#[derive(Debug)]
pub struct MarketMaker {
    inventory: f64,
    pub inventory_limit: f64,
    pub spread: f64,
}

impl MarketMaker {
    pub fn new(inventory_limit: f64, spread: f64) -> Self {
        Self { inventory: 0.0, inventory_limit, spread }
    }

    pub fn inventory(&self) -> f64 {
        self.inventory
    }

    pub fn quote(&self, best_bid: f64, best_ask: f64, imbalance: f64) -> (f64, f64) {
        let mid = (best_bid + best_ask) / 2.0;
        let base_spread = if best_ask > best_bid { best_ask - best_bid } else { self.spread };
        let adj = imbalance * base_spread;
        let bid = mid - self.spread / 2.0 + adj;
        let ask = mid + self.spread / 2.0 + adj;
        (bid, ask)
    }

    pub fn trade(&mut self, qty: f64, side: Side) {
        match side {
            Side::Buy => self.inventory += qty,
            Side::Sell => self.inventory -= qty,
        }
        if self.inventory.abs() > self.inventory_limit {
            println!("Warning: inventory limit exceeded: {}", self.inventory);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}
