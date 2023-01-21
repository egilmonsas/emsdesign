use crate::crs::{CrossSection, CrossSectionClass, CrossSectionClassCase};
use crate::erc::NSEN_1993::{BuckleCurve, _compute_lamba, _compute_phi, f_6_47, f_6_49};
use crate::mat::steel::Steel;
use crate::{crs::heb::CrsHEB, mat::Material};
use serde_json::{json, Value};

use crate::{Axis, LimitStateType};

pub struct ColumnBeam {
    pub crs: Box<dyn CrossSection>,
    pub mat: Steel,
}
#[allow(clippy::derivable_impls)]
impl Default for ColumnBeam {
    fn default() -> Self {
        Self {
            crs: Box::<CrsHEB>::default(),
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
    #[allow(non_snake_case)]
    #[must_use]
    pub fn V_pl(&self, axis: Axis, limit_state_type: &LimitStateType) -> f64 {
        self.mat.f_y(limit_state_type) * self.crs.area_shear(axis) / 3f64.sqrt()
    }

    #[must_use]
    pub fn buckle_cap(
        &self,
        lk: f64,
        axis: Axis,
        buckle_curve: &BuckleCurve,
        limit_state_type: &LimitStateType,
    ) -> f64 {
        {
            // Eurocode 1993 buckling
            let ncr = self.euler_load(lk, axis);
            let lambda = _compute_lamba(self.crs.area(), self.mat.f_y(&LimitStateType::K), ncr);
            let phi = _compute_phi(buckle_curve.alpha(), lambda);
            let khi_buckle_reduction_factor = f_6_49(phi, lambda);
            f_6_47(
                khi_buckle_reduction_factor,
                self.crs.area(),
                self.mat.f_y(&LimitStateType::K),
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
        match self.crs.variant() {
            crate::crs::Variant::HEB => {
                json!({
                    "EA": self.EA(),
                    "EI_y": self.EI(Axis::Y),
                    "EI_z": self.EI(Axis::Z),
                    "Cross_section_class_web_bending": self.crs.cross_section_class(self.mat.epsilon(), CrossSectionClassCase::WebBending),
                    "Cross_section_class_web_compression": self.crs.cross_section_class(self.mat.epsilon(), CrossSectionClassCase::WebCompression),
                    "Cross_section_class_flange_compression": self.crs.cross_section_class(self.mat.epsilon(), CrossSectionClassCase::FlangeCompression),
                    "N_pl_k": self.N_pl( &LimitStateType::K),
                    "V_pl_y_k": self.V_pl(Axis::Y, &LimitStateType::K),
                    "M_el_y_k":  self.M_el(Axis::Y,&LimitStateType::K),
                    "M_pl_y_k": self.M_pl(Axis::Y,&LimitStateType::K),
                    "V_pl_z_k": self.V_pl(Axis::Z, &LimitStateType::K),
                    "M_el_z_k":  self.M_el(Axis::Z,&LimitStateType::K),
                    "M_pl_z_k": self.M_pl(Axis::Z,&LimitStateType::K),
                    "N_pl_d": self.N_pl(&LimitStateType::D),
                    "V_pl_y_d": self.V_pl(Axis::Y, &LimitStateType::D),
                    "M_el_y_d":  self.M_el(Axis::Y,&LimitStateType::D),
                    "M_pl_y_d": self.M_pl(Axis::Y,&LimitStateType::D),
                    "V_pl_z_d": self.V_pl(Axis::Z, &LimitStateType::D),
                    "M_el_z_d":  self.M_el(Axis::Z,&LimitStateType::D),
                    "M_pl_z_d": self.M_pl(Axis::Z,&LimitStateType::D),
                })
            }
            crate::crs::Variant::CHS => {
                json!({
                    "EA": self.EA(),
                    "EI": self.EI(Axis::Y),
                    "Cross_section_class": self.crs.cross_section_class(self.mat.epsilon(), CrossSectionClassCase::None),
                    "N_pl_k": self.N_pl( &LimitStateType::K),
                    "N_pl_d": self.N_pl(&LimitStateType::D),
                    "V_pl_k": self.V_pl(Axis::Y, &LimitStateType::K),
                    "V_pl_d": self.V_pl(Axis::Y, &LimitStateType::D),
                    "M_el_k":  self.M_el(Axis::Y,&LimitStateType::K),
                    "M_el_d":  self.M_el(Axis::Y,&LimitStateType::D),
                    "M_pl_k": self.M_pl(Axis::Y,&LimitStateType::K),
                    "M_pl_d":  self.M_el(Axis::Y,&LimitStateType::D),
                })
            }
        }
    }

    #[must_use]
    pub fn cross_section_class(&self, case: CrossSectionClassCase) -> CrossSectionClass {
        self.crs.cross_section_class(self.mat.epsilon(), case)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn axial_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.N_pl(&LimitStateType::K), 923_000.0);
    }
    #[test]
    fn axial_cap_circle() {
        let mmb = ColumnBeam {
            crs: Box::<CrsHEB>::default(),
            ..Default::default()
        };
        assert_zeq!(mmb.N_pl(&LimitStateType::K), 923_000.0);
    }

    #[test]
    fn moment_cap() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.M_el(Axis::Y, &LimitStateType::K), 31_914_500.0);
    }

    #[test]
    fn ea() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EA(), 546_000_000.0);
    }
    #[test]
    fn ei() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI(Axis::Y), 945_000_000_000.0);
    }
    #[test]
    fn euler_load() {
        let mmb = ColumnBeam::default();
        let lk = 10000.0;
        assert_zeq!(mmb.euler_load(lk, Axis::Z), 34_612.702_634_620_38);
    }
}
