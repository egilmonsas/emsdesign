use std::f64::consts::PI;

use super::CrossSection;

pub struct CrsCircle {
    r: f64,
}

impl CrsCircle {
    pub fn new(r: f64) -> Self {
        Self { r }
    }

    pub fn default() -> Self {
        Self { r: 50.0 }
    }
}

impl CrossSection for CrsCircle {
    fn area(&self) -> f64 {
        PI * self.r.powi(2)
    }

    fn centroid(&self) -> (f64, f64) {
        (self.r, self.r)
    }

    fn I(&self) -> (f64, f64) {
        (PI / 4.0 * self.r.powi(4), PI / 2.0 * self.r.powi(4))
    }

    fn w(&self) -> (f64, f64) {
        (PI / 4.0 * self.r.powi(3), PI / 2.0 * self.r.powi(3))
    }
}
