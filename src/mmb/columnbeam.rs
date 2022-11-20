use crate::crs::rect::CrsRect;
use crate::crs::CrossSection;
use crate::erc::NSEN_1993::*;
use crate::mat::steel::Steel;

pub struct ColumnBeam {
    pub crs: Box<dyn CrossSection>,
    pub mat: Steel,
    pub len: f64,
}

impl Default for ColumnBeam {
    fn default() -> Self {
        Self {
            crs: Box::new(CrsRect::default()),
            mat: Steel::default(),
            len: 1.0,
        }
    }
}
impl ColumnBeam {
    pub fn new(crs: Box<dyn CrossSection>, mat: Steel, len: f64) -> Self {
        Self { crs, mat, len }
    }

    pub fn axial_cap(&self) -> f64 {
        self.mat.fy * self.crs.area()
    }

    pub fn buckle_cap(&self, lky: f64, lkz: f64) -> f64 {
        {
            // Eurocode 1993 buckling
            let gamma_1 = 1.15;
            let alpha = _get_alpha(BuckleCurve::C);
            let eulerloads = self.euler_load((lky, lkz));
            let ncr = eulerloads.0.min(eulerloads.1);
            let lambda = _compute_lamba(self.crs.area(), self.mat.fy, ncr);
            let phi = _compute_phi(alpha, lambda);
            let khi = f_6_49(phi, lambda);
            f_6_47(khi, self.crs.area(), self.mat.fy, gamma_1)
        }
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
    use crate::{crs::circle::CrsCircle, zeq::Zeq};

    #[test]
    fn axial_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.axial_cap(), 3_550_000.0)
    }
    #[test]
    fn axial_cap_circle() {
        let mmb = ColumnBeam {
            crs: Box::new(CrsCircle::default()),
            ..Default::default()
        };
        assert_zeq!(mmb.axial_cap(), 2_788_163.480060)
    }

    #[test]
    fn moment_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.moment_cap().0, 59_166_666.66666)
    }

    #[test]
    fn ea() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EA(), 2_100_000_000.0)
    }
    #[test]
    fn ei() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI().0, 1_750_000_000_000.0)
    }
    #[test]
    fn euler_load() {
        let mmb = ColumnBeam::default();
        let lky = 10000.0;
        let lkz = 10000.0;
        assert_zeq!(mmb.euler_load((lky, lkz)).0, 172_718.077019)
    }
}
