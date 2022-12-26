use crate::Axis;

use super::CrossSection;

pub struct CrsBox {
    y: f64,
    z: f64,
    t: f64,
}

impl CrsBox {
    pub fn new(y: f64, z: f64, t: f64) -> Self {
        Self { y, z, t }
    }

    pub fn default() -> Self {
        Self {
            y: 100.0,
            z: 100.0,
            t: 10.0,
        }
    }

    fn y_inner(&self) -> f64 {
        self.y - 2.0 * self.t
    }
    fn z_inner(&self) -> f64 {
        self.z - 2.0 * self.t
    }
}
impl CrossSection for CrsBox {
    fn width(&self) -> f64 {
        self.y
    }
    fn height(&self) -> f64 {
        self.z
    }
    fn area(&self) -> f64 {
        self.y * self.z - self.y_inner() * self.z_inner()
    }

    #[allow(non_snake_case)]
    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => (self.y * self.z.powi(3) - self.y_inner() * self.z_inner().powi(3)) / 12.0,
            Axis::Z => (self.z * self.y.powi(3) - self.z_inner() * self.y_inner().powi(3)) / 12.0,
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
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn create_cross_section() {
        let width = 50.0;
        let height = 100.0;
        let thickness = 1.0;
        let crs = CrsBox::new(width, height, thickness);

        assert_zeq!(crs.y, width);
        assert_zeq!(crs.z, height);
        assert_zeq!(crs.t, thickness);
    }

    #[test]
    fn area_cross_section() {
        let width = 50.0;
        let height = 100.0;
        let thickness = 10.0;
        let crs = CrsBox::new(width, height, thickness);

        let result = crs.area();
        let expected_result = 50.0 * 100.0 - 30.0 * 80.0;

        assert_zeq!(result, expected_result)
    }

    #[test]
    fn centroid_cross_section() {
        let width = 50.0;
        let height = 100.0;
        let thickness = 10.0;
        let crs = CrsBox::new(width, height, thickness);

        let centroid = crs.centroid();
        assert_zeq!(centroid.0, 25.0);
        assert_zeq!(centroid.1, 50.0);
    }

    #[test]
    fn second_moment_of_area() {
        let width = 50.0;
        let height = 100.0;
        let thickness = 10.0;
        let crs = CrsBox::new(width, height, thickness);

        assert_zeq!(crs.I(Axis::Y), 2_886_666.666666);
        assert_zeq!(crs.I(Axis::Z), 861_666.666666);
    }

    #[test]
    fn bending_moment() {
        let width = 50.0;
        let height = 100.0;
        let thickness = 10.0;
        let crs = CrsBox::new(width, height, thickness);

        assert_zeq!(crs.w_el(Axis::Y), 2_886_666.666666 / 50.0);
        assert_zeq!(crs.w_el(Axis::Z), 861_666.666666 / 25.0);
    }
}
