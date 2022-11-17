use crate::crs::rect::CrsRect;
use crate::mat::steel::Steel;

pub struct ColumnBeam {
    pub crs: CrsRect,
    pub mat: Steel,
    pub len: f64,
}

impl Default for ColumnBeam {
    fn default() -> Self {
        Self {
            crs: CrsRect::default(),
            mat: Steel::default(),
            len: 1.0,
        }
    }
}
impl ColumnBeam {
    pub fn new(crs: CrsRect, mat: Steel, len: f64) -> Self {
        Self { crs, mat, len }
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
    pub fn euler_load(&self, lks: (f64, f64)) -> (f64, f64) {
        let ei = self.EI();
        (
            ei.0 * (std::f64::consts::PI / lks.0).powi(2),
            ei.1 * (std::f64::consts::PI / lks.1).powi(2),
        )
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
        let mmb = ColumnBeam {
            crs,
            mat,
            ..Default::default()
        };
        assert_zeq!(mmb.axial_cap(), 3_550_000.0)
    }

    #[test]
    fn moment_cap() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam {
            crs,
            mat,
            ..Default::default()
        };
        assert_zeq!(mmb.moment_cap().0, 59_166_666.66666)
    }

    #[test]
    fn ea() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam {
            crs,
            mat,
            ..Default::default()
        };
        assert_zeq!(mmb.EA(), 2_100_000_000.0)
    }
    #[test]
    fn ei() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam {
            crs,
            mat,
            ..Default::default()
        };
        assert_zeq!(mmb.EI().0, 1_750_000_000_000.0)
    }
    #[test]
    fn euler_load() {
        let crs = CrsRect::new(100.0, 100.0);
        let mat = Steel::default();
        let mmb = ColumnBeam {
            crs,
            mat,
            len: 10000.0,
        };
        let lky = 10000.0;
        let lkz = 10000.0;
        assert_zeq!(mmb.euler_load((lky, lkz)).0, 172_718.077019)
    }
}
