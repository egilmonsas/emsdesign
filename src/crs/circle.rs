use std::f64::consts::PI;

use crate::Axis;

use super::CrossSection;

pub struct Circle {
    d: f64,
}

impl Circle {
    #[must_use]
    pub const fn new(d: f64) -> Self {
        Self { d }
    }

    fn r(&self) -> f64 {
        self.d / 2.0
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new(100.0)
    }
}

impl CrossSection for Circle {
    fn width(&self) -> f64 {
        self.d
    }
    fn height(&self) -> f64 {
        self.d
    }
    fn area(&self) -> f64 {
        PI * self.r().powi(2)
    }

    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => PI / 4.0 * self.r().powi(4),
        }
    }

    fn w_el(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => PI / 4.0 * self.r().powi(3),
        }
    }

    fn w_pl(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => (4.0 / 3.0) * self.r().powi(3),
        }
    }

    fn area_shear(&self, _axis: Axis) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn create_cross_section() {
        let diameter = 100.0;
        let crs = Circle::new(diameter);

        assert_zeq!(crs.d, diameter);
    }

    #[test]
    fn area_cross_section() {
        let diameter = 100.0;
        let crs = Circle::new(diameter);

        let result = crs.area();
        let expected_result = PI * 50.0f64.powi(2);

        assert_zeq!(result, expected_result);
    }

    #[test]
    fn centroid_cross_section() {
        let diameter = 100.0;
        let crs = Circle::new(diameter);

        let centroid = crs.centroid();
        assert_zeq!(centroid.0, 50.0);
        assert_zeq!(centroid.1, 50.0);
    }

    #[test]
    fn second_moment_of_area() {
        let diameter = 100.0;
        let crs = Circle::new(diameter);

        assert_zeq!(crs.I(Axis::Y), PI / 4.0 * 50.0f64.powi(4));
        assert_zeq!(crs.I(Axis::Z), PI / 4.0 * 50.0f64.powi(4));
    }

    #[test]
    fn bending_moment() {
        let diameter = 100.0;
        let crs = Circle::new(diameter);

        assert_zeq!(crs.w_el(Axis::Y), PI / 4.0 * 50.0f64.powi(3));
        assert_zeq!(crs.w_el(Axis::Z), PI / 4.0 * 50.0f64.powi(3));
    }
}
