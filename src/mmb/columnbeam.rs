use crate::crs::rect::CrsRect;
use crate::mat::steel::Steel;

pub struct ColumnBeam {
    pub crs: CrsRect,
    pub mat: Steel,
}

impl ColumnBeam {
    pub fn new(crs: CrsRect, mat: Steel) -> Self {
        Self { crs, mat }
    }

    pub fn axial_cap(&self) -> f64 {
        self.mat.fy * self.crs.area()
    }

    pub fn moment_cap(&self) -> (f64, f64) {
        let w = self.crs.w();
        (w.0 * self.mat.fy, w.1 * self.mat.fy)
    }
    #[allow(non_snake_case)]
    pub fn EA(&self) -> f64 {
        self.mat.E * self.crs.area()
    }
    #[allow(non_snake_case)]
    pub fn EI(&self) -> (f64, f64) {
        let I = self.crs.I();
        (I.0 * self.mat.E, I.1 * self.mat.E)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn axial_cap() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam::new(crs, mat);
        assert_zeq!(mmb.axial_cap(), 3_550_000.0)
    }

    #[test]
    fn moment_cap() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam::new(crs, mat);
        assert_zeq!(mmb.moment_cap().0, 59_166_666.66666)
    }

    #[test]
    fn ea() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam::new(crs, mat);
        assert_zeq!(mmb.EA(), 2_100_000_000.0)
    }
    #[test]
    fn ei() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam::new(crs, mat);
        assert_zeq!(mmb.EI().0, 1_750_000_000_000.0)
    }
}
