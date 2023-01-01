use std::f64::consts::PI;

use crate::Axis;

use super::CrossSection;

pub struct CrsTube {
    d: f64,
    t: f64,
}

impl CrsTube {
    #[must_use]
    pub const fn new(d: f64, t: f64) -> Self {
        Self { d, t }
    }

    fn r(&self) -> f64 {
        self.d / 2.0
    }
    fn r_inner(&self) -> f64 {
        self.d / 2.0 - self.t
    }
}

impl Default for CrsTube {
    fn default() -> Self {
        Self { d: 100.0, t: 10.0 }
    }
}

impl CrossSection for CrsTube {
    fn width(&self) -> f64 {
        self.d
    }
    fn height(&self) -> f64 {
        self.d
    }
    fn area(&self) -> f64 {
        PI * (self.r().powi(2) - self.r_inner().powi(2))
    }

    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => PI / 4.0 * (self.r().powi(4) - self.r_inner().powi(4)),
        }
    }

    fn w_el(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => self.I(axis) / self.r(),
        }
    }

    fn w_pl(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => (4.0 / 3.0) * (self.r().powi(3) - self.r_inner().powi(3)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

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

        assert_zeq!(result, expected_result);
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

        assert_zeq!(crs.I(Axis::Y), 2_898_119.222_936);
        assert_zeq!(crs.I(Axis::Z), 2_898_119.222_936);
    }

    #[test]
    fn bending_moment() {
        let diameter = 100.0;
        let thickness = 10.0;
        let crs = CrsTube::new(diameter, thickness);

        assert_zeq!(crs.w_el(Axis::Y), 57_962.384_458);
        assert_zeq!(crs.w_el(Axis::Z), 57_962.384_458);
    }
}
