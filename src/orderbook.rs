use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

impl OrderBook {
    pub fn update_bid(&mut self, price: f64, qty: f64) {
        let p = OrderedFloat(price);
        if qty == 0.0 {
            self.bids.remove(&p);
        } else {
            self.bids.insert(p, qty);
        }
    }

    pub fn update_ask(&mut self, price: f64, qty: f64) {
        let p = OrderedFloat(price);
        if qty == 0.0 {
            self.asks.remove(&p);
        } else {
            self.asks.insert(p, qty);
        }
    }

    pub fn best_bid(&self) -> Option<(f64, f64)> {
        self.bids.iter().rev().next().map(|(p, q)| (p.0, *q))
    }

    pub fn best_ask(&self) -> Option<(f64, f64)> {
        self.asks.iter().next().map(|(p, q)| (p.0, *q))
    }

    pub fn imbalance(&self) -> Option<f64> {
        let bid_vol: f64 = self.bids.values().copied().sum();
        let ask_vol: f64 = self.asks.values().copied().sum();
        if bid_vol + ask_vol > 0.0 {
            Some((bid_vol - ask_vol) / (bid_vol + ask_vol))
        } else {
            None
        }
    }
}
