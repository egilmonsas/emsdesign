use crate::Axis;

use super::CrossSection;

pub struct CrsRect {
    y: f64,
    z: f64,
}

impl CrsRect {
    #[must_use] pub fn new(y: f64, z: f64) -> Self {
        Self { y, z }
    }

    #[must_use] pub fn default() -> Self {
        Self { y: 100.0, z: 100.0 }
    }
}
impl CrossSection for CrsRect {
    fn width(&self) -> f64 {
        self.y
    }
    fn height(&self) -> f64 {
        self.z
    }
    fn area(&self) -> f64 {
        self.y * self.z
    }

    #[allow(non_snake_case)]
    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.y * self.z.powi(3) / 12.0,
            Axis::Z => self.z * self.y.powi(3) / 12.0,
        }
    }

    fn w_el(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.I(axis) / (self.z / 2.0),
            Axis::Z => self.I(axis) / (self.y / 2.0),
        }
    }
    fn w_pl(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => (1.0 / 4.0) * self.width() * self.height().powi(2),
            Axis::Z => (1.0 / 4.0) * self.height() * self.width().powi(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn create_rectangular_cross_section() {
        let a = 2.0;
        let b = 3.0;
        let crs = CrsRect::new(a, b);

        assert_zeq!(crs.y, a);
        assert_nzeq!(crs.z, a);
    }

    #[test]
    fn area_rectangular_cross_section() {
        let a = 2.0;
        let b = 3.0;
        let crs = CrsRect::new(a, b);

        let result = crs.area();
        let expected_result = 6.0;

        assert_zeq!(result, expected_result)
    }

    #[test]
    fn centroid_rectangular_cross_section() {
        let a = 2.0;
        let b = 3.0;
        let crs = CrsRect::new(a, b);

        let centroid = crs.centroid();
        assert_zeq!(centroid.0, 1.0);
        assert_zeq!(centroid.1, 1.5);
    }

    #[test]
    fn second_moment_of_area_rectangular_cross_section() {
        let width = 100.0;
        let height = 300.0;
        let crs = CrsRect::new(width, height);

        assert_zeq!(crs.I(Axis::Y), 225_000_000.0);
        assert_zeq!(crs.I(Axis::Z), 25_000_000.0);
    }

    #[test]
    fn bending_moment_rectangular_cross_section() {
        let width = 100.0;
        let height = 300.0;
        let crs = CrsRect::new(width, height);

        assert_zeq!(crs.w_el(Axis::Y), 1_500_000.0);
        assert_zeq!(crs.w_el(Axis::Z), 500_000.0);
    }
}
