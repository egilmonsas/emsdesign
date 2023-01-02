use crate::crs::CrossSection;
use crate::erc::NSEN_1993::{BuckleCurve, _compute_lamba, _compute_phi, f_6_47, f_6_49};
use crate::mat::steel::Steel;
use crate::{crs::rect::Rect, mat::Material};
use serde_json::{json, Value};

use crate::{Axis, LimitStateType};

pub struct ColumnBeam {
    pub crs: Box<dyn CrossSection>,
    pub mat: Steel,
}

impl Default for ColumnBeam {
    fn default() -> Self {
        Self {
            crs: Box::new(Rect::default()),
            mat: Steel::default(),
        }
    }
}
impl ColumnBeam {
    #[must_use]
    pub fn new(crs: Box<dyn CrossSection>, mat: Steel) -> Self {
        Self { crs, mat }
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn N_pl(&self, limit_state_type: &LimitStateType) -> f64 {
        self.mat.f_y(limit_state_type) * self.crs.area()
    }

    #[must_use]
    pub fn buckle_cap(&self, lk: f64, axis: Axis, limit_state_type: &LimitStateType) -> f64 {
        {
            // Eurocode 1993 buckling
            let ncr = self.euler_load(lk, axis);
            let lambda = _compute_lamba(self.crs.area(), self.mat.f_y(limit_state_type), ncr);
            let phi = _compute_phi(BuckleCurve::C.alpha(), lambda);
            let khi_buckle_reduction_factor = f_6_49(phi, lambda);
            f_6_47(
                khi_buckle_reduction_factor,
                self.crs.area(),
                self.mat.f_y(limit_state_type),
                self.mat.gamma_m1(limit_state_type),
            )
        }
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_el(&self, axis: Axis, limit_state_type: &LimitStateType) -> f64 {
        self.crs.w_el(axis) * self.mat.f_y(limit_state_type)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_pl(&self, axis: Axis, limit_state_type: &LimitStateType) -> f64 {
        self.crs.w_pl(axis) * self.mat.f_y(limit_state_type)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn EA(&self) -> f64 {
        self.mat.E() * self.crs.area()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn EI(&self, axis: Axis) -> f64 {
        let I = self.crs.I(axis);
        I * self.mat.E()
    }

    #[must_use]
    pub fn euler_load(&self, lk: f64, axis: Axis) -> f64 {
        self.EI(axis) * (std::f64::consts::PI / lk).powi(2)
    }

    #[must_use]
    pub fn json(&self) -> Value {
        let jsonout = json!({
            "EA": self.EA(),
            "EI_y": self.EI(Axis::Y),
            "EI_z": self.EI(Axis::Z),
            "N_pl_k": self.N_pl( &LimitStateType::K),
            "M_el_y_k":  self.M_el(Axis::Y,&LimitStateType::K),
            "M_pl_y_k": self.M_pl(Axis::Y,&LimitStateType::K),
            "M_el_z_k":  self.M_el(Axis::Z,&LimitStateType::K),
            "M_pl_z_k": self.M_pl(Axis::Z,&LimitStateType::K),
            "N_pl_d": self.N_pl(&LimitStateType::D),
            "M_el_y_d":  self.M_el(Axis::Y,&LimitStateType::D),
            "M_pl_y_d": self.M_pl(Axis::Y,&LimitStateType::D),
            "M_el_z_d":  self.M_el(Axis::Z,&LimitStateType::D),
            "M_pl_z_d": self.M_pl(Axis::Z,&LimitStateType::D),

        });
        jsonout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{crs::circle::Circle, zequality::Zeq};

    #[test]
    fn axial_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.N_pl(&LimitStateType::K), 3_550_000.0);
    }
    #[test]
    fn axial_cap_circle() {
        let mmb = ColumnBeam {
            crs: Box::new(Circle::default()),
            ..Default::default()
        };
        assert_zeq!(mmb.N_pl(&LimitStateType::K), 2_788_163.480_060);
    }

    #[test]
    fn moment_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.M_el(Axis::Y, &LimitStateType::K), 59_166_666.666_66);
    }

    #[test]
    fn ea() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EA(), 2_100_000_000.0);
    }
    #[test]
    fn ei() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI(Axis::Y), 1_750_000_000_000.0);
    }
    #[test]
    fn euler_load() {
        let mmb = ColumnBeam::default();
        let lk = 10000.0;
        assert_zeq!(mmb.euler_load(lk, Axis::Z), 172_718.077_019);
    }
}
