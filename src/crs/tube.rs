use std::f64::consts::PI;

use super::CrossSection;

pub struct CrsTube {
    d: f64,
    t: f64,
}

impl CrsTube {
    pub fn new(d: f64, t: f64) -> Self {
        Self { d, t }
    }

    pub fn default() -> Self {
        Self { d: 100.0, t: 10.0 }
    }

    fn r(&self) -> f64 {
        self.d / 2.0
    }
    fn r_inner(&self) -> f64 {
        self.d / 2.0 - self.t
    }
}

impl CrossSection for CrsTube {
    fn area(&self) -> f64 {
        PI * (self.r().powi(2) - self.r_inner().powi(2))
    }

    fn centroid(&self) -> (f64, f64) {
        (self.r(), self.r())
    }

    fn I(&self) -> (f64, f64) {
        (
            PI / 4.0 * (self.r().powi(4) - self.r_inner().powi(4)),
            PI / 4.0 * (self.r().powi(4) - self.r_inner().powi(4)),
        )
    }

    fn w(&self) -> (f64, f64) {
        let i = self.I();
        (i.0 / self.r(), i.1 / self.r())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn create_cross_section() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        assert_zeq!(crs.d, diameter);
        assert_nzeq!(crs.t, diameter);
    }

    #[test]
    fn area_cross_section() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        let result = crs.area();
        let expected_result = PI * (50.0 * 50.0 - 40.0 * 40.0);

        assert_zeq!(result, expected_result)
    }

    #[test]
    fn centroid_cross_section() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        let centroid = crs.centroid();
        assert_zeq!(centroid.0, 50.0);
        assert_zeq!(centroid.1, 50.0);
    }

    #[test]
    fn second_moment_of_area() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        let inertia = crs.I();
        assert_zeq!(inertia.0, 2_898_119.222936);
        assert_zeq!(inertia.1, 2_898_119.222936);
    }

    #[test]
    fn bending_moment() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        let inertia = crs.w();
        assert_zeq!(inertia.0, 57_962.384458);
        assert_zeq!(inertia.1, 57_962.384458);
    }
}
