#[derive(Default)]
pub struct StatArb {
    window: usize,
    history: Vec<(f64, f64)>,
}

impl StatArb {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            history: Vec::new(),
        }
    }

    pub fn update(&mut self, p1: f64, p2: f64) -> Option<f64> {
        self.history.push((p1, p2));
        if self.history.len() > self.window {
            self.history.remove(0);
        }
        let diffs: Vec<f64> = self.history.iter().map(|(a, b)| a - b).collect();
        if diffs.len() < 2 {
            return None;
        }
        let mean: f64 = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let var: f64 = diffs.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / diffs.len() as f64;
        let std = var.sqrt();
        let last = *diffs.last().unwrap();
        if std > 0.0 {
            Some((last - mean) / std)
        } else {
            None
        }
    }
}
