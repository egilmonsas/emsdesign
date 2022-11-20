use std::f64::consts::PI;

use super::CrossSection;

pub struct CrsCircle {
    d: f64,
}

impl CrsCircle {
    pub fn new(d: f64) -> Self {
        Self { d }
    }

    pub fn default() -> Self {
        Self { d: 100.0 }
    }
    fn r(&self) -> f64 {
        self.d / 2.0
    }
}

impl CrossSection for CrsCircle {
    fn area(&self) -> f64 {
        PI * self.r().powi(2)
    }

    fn centroid(&self) -> (f64, f64) {
        (self.r(), self.r())
    }

    fn I(&self) -> (f64, f64) {
        (PI / 4.0 * self.r().powi(4), PI / 4.0 * self.r().powi(4))
    }

    fn w(&self) -> (f64, f64) {
        (PI / 4.0 * self.r().powi(3), PI / 4.0 * self.r().powi(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn create_cross_section() {
        let diameter = 100.0;
        let crs = CrsCircle::new(diameter);

        assert_zeq!(crs.d, diameter);
    }

    #[test]
    fn area_cross_section() {
        let diameter = 100.0;
        let crs = CrsCircle::new(diameter);

        let result = crs.area();
        let expected_result = PI * 50.0f64.powi(2);

        assert_zeq!(result, expected_result)
    }

    #[test]
    fn centroid_cross_section() {
        let diameter = 100.0;
        let crs = CrsCircle::new(diameter);

        let centroid = crs.centroid();
        assert_zeq!(centroid.0, 50.0);
        assert_zeq!(centroid.1, 50.0);
    }

    #[test]
    fn second_moment_of_area() {
        let diameter = 100.0;
        let crs = CrsCircle::new(diameter);

        let inertia = crs.I();
        assert_zeq!(inertia.0, PI / 4.0 * 50.0f64.powi(4));
        assert_zeq!(inertia.1, PI / 4.0 * 50.0f64.powi(4));
    }

    #[test]
    fn bending_moment() {
        let diameter = 100.0;
        let crs = CrsCircle::new(diameter);

        let inertia = crs.w();
        assert_zeq!(inertia.0, PI / 4.0 * 50.0f64.powi(3));
        assert_zeq!(inertia.1, PI / 4.0 * 50.0f64.powi(3));
    }
}
