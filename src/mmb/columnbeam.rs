use crate::crs::rect::CrsRect;
use crate::crs::CrossSection;
use crate::erc::NSEN_1993::*;
use crate::mat::steel::Steel;
use serde_json::{json, Value};

use crate::Axis;

pub struct ColumnBeam {
    pub crs: Box<dyn CrossSection>,
    pub mat: Steel,
}

impl Default for ColumnBeam {
    fn default() -> Self {
        Self {
            crs: Box::new(CrsRect::default()),
            mat: Steel::default(),
        }
    }
}
impl ColumnBeam {
    pub fn new(crs: Box<dyn CrossSection>, mat: Steel) -> Self {
        Self { crs, mat }
    }

    pub fn N_pl(&self) -> f64 {
        self.mat.fy * self.crs.area()
    }

    pub fn buckle_cap(&self, lk: f64, axis: Axis) -> f64 {
        {
            // Eurocode 1993 buckling
            let gamma_1 = 1.15;
            let ncr = self.euler_load(lk, axis);
            let lambda = _compute_lamba(self.crs.area(), self.mat.fy, ncr);
            let phi = _compute_phi(BuckleCurve::C.alpha(), lambda);
            let khi = f_6_49(phi, lambda);
            f_6_47(khi, self.crs.area(), self.mat.fy, gamma_1)
        }
    }
    #[allow(non_snake_case)]
    pub fn M_el(&self, axis: Axis) -> f64 {
        self.crs.w_el(axis) * self.mat.fy
    }
    #[allow(non_snake_case)]
    pub fn M_pl(&self, axis: Axis) -> f64 {
        self.crs.w_pl(axis) * self.mat.fy
    }
    #[allow(non_snake_case)]
    pub fn EA(&self) -> f64 {
        self.mat.E * self.crs.area()
    }
    #[allow(non_snake_case)]
    pub fn EI(&self, axis: Axis) -> f64 {
        let I = self.crs.I(axis);
        I * self.mat.E
    }

    pub fn euler_load(&self, lk: f64, axis: Axis) -> f64 {
        self.EI(axis) * (std::f64::consts::PI / lk).powi(2)
    }

    pub fn json(&self) -> Value {
        let jsonout = json!({
            "EA": self.EA(),
            "EI_y": self.EI(Axis::Y),
            "EI_z": self.EI(Axis::Z),
            "N_pl": self.N_pl(),
            "M_el_y":  self.M_el(Axis::Y),
            "M_pl_y": self.M_pl(Axis::Y),
            "M_el_z":  self.M_el(Axis::Z),
            "M_pl_z": self.M_pl(Axis::Z),

        });
        jsonout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crs::circle::CrsCircle, zeq::Zeq};

    #[test]
    fn axial_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.N_pl(), 3_550_000.0)
    }
    #[test]
    fn axial_cap_circle() {
        let mmb = ColumnBeam {
            crs: Box::new(CrsCircle::default()),
            ..Default::default()
        };
        assert_zeq!(mmb.N_pl(), 2_788_163.480060)
    }

    #[test]
    fn moment_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.M_el(Axis::Y), 59_166_666.66666)
    }

    #[test]
    fn ea() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EA(), 2_100_000_000.0)
    }
    #[test]
    fn ei() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI(Axis::Y), 1_750_000_000_000.0)
    }
    #[test]
    fn euler_load() {
        let mmb = ColumnBeam::default();
        let lk = 10000.0;
        assert_zeq!(mmb.euler_load(lk, Axis::Z), 172_718.077019)
    }
}
