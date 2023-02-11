use crate::crs::{CrossSection, CrossSectionClass, CrossSectionClassCase};
use crate::erc::NSEN_1993::{self, BuckleCurve, LTBCurve, Table6_7, TableB1};
use crate::load::loadcase::LoadCase;
use crate::mat::steel::Steel;
use crate::{crs::heb::CrsHEB, mat::Material};
use serde::Serialize;
use serde_json::{json, Value};

use crate::{Axis, LimitStateType};

#[derive(Serialize)]
pub struct DesignChecks {
    pub util_6_2: f64,
    pub util_6_46_y: f64,
    pub util_6_46_z: f64,
    pub util_6_61: f64,
    pub util_6_62: f64,
}
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
    #[must_use]
    pub fn dc(
        &self,
        design_load: &LoadCase,
        c_my: f64,
        c_mz: f64,
        mu_cr: f64,
        lk_y: f64,
        lk_z: f64,
        lk_ltb: f64,
        buckle_curve_y: &BuckleCurve,
        buckle_curve_z: &BuckleCurve,
        ltb_curve: &LTBCurve,
    ) -> DesignChecks {
        DesignChecks {
            util_6_2: self.dc_6_2(design_load),
            util_6_46_y: self.dc_6_46(design_load, lk_y, &Axis::Y, buckle_curve_y),
            util_6_46_z: self.dc_6_46(design_load, lk_z, &Axis::Z, buckle_curve_z),
            util_6_61: self.dc_6_61(
                design_load,
                c_my,
                c_mz,
                mu_cr,
                lk_ltb,
                buckle_curve_y,
                ltb_curve,
            ),
            util_6_62: self.dc_6_62(
                design_load,
                c_my,
                c_mz,
                mu_cr,
                lk_ltb,
                buckle_curve_z,
                ltb_curve,
            ),
        }
    }
    #[must_use]
    pub fn dc_6_2(&self, design_load: &LoadCase) -> f64 {
        NSEN_1993::f_6_2(
            design_load.N,
            design_load.My,
            design_load.Mz,
            self.N_pl(&LimitStateType::D),
            self.M_pl(&Axis::Y, &LimitStateType::D),
            self.M_pl(&Axis::Z, &LimitStateType::D),
        )
    }
    #[must_use]
    pub fn dc_6_46(
        &self,
        design_load: &LoadCase,
        lk: f64,
        axis: &Axis,
        buckle_curve: &BuckleCurve,
    ) -> f64 {
        let n_b_rd = self.buckle_cap(lk, axis, buckle_curve, &LimitStateType::D);
        design_load.N / n_b_rd
    }
    #[must_use]
    pub fn dc_6_61(
        &self,
        design_load: &LoadCase,
        c_my: f64,
        c_mz: f64,
        mu_cr: f64,
        lk: f64,
        buckle_curve_y: &BuckleCurve,
        ltb_curve: &LTBCurve,
    ) -> f64 {
        let khi_buckle_reduction_factor = self.khi(lk, &Axis::Y, buckle_curve_y);
        let table6_7 = Table6_7::from_crs_class(
            &self.crs,
            &self.cross_section_class(&CrossSectionClassCase::WebCompression),
        );
        let tableb_1 = TableB1::from_crs_class(
            self.euler_load(lk, &Axis::Y),
            self.euler_load(lk, &Axis::Z),
            c_my,
            c_mz,
            self,
            &self.cross_section_class(&CrossSectionClassCase::WebCompression),
            buckle_curve_y,
            design_load.N,
        );
        let util_n = NSEN_1993::f_6_61_util_N(
            design_load.N,
            khi_buckle_reduction_factor,
            self.N_pl(&LimitStateType::K),
            self.mat.gamma_m1(&LimitStateType::D),
        );
        let util_my = NSEN_1993::f_6_61_util_My(
            design_load.My,
            table6_7.delta_My_Ed,
            table6_7.Wy * self.mat.f_y(&LimitStateType::K),
            tableb_1.k_yy,
            self.khi_lt(lk, ltb_curve, mu_cr),
            self.mat.gamma_m1(&LimitStateType::D),
        );
        let util_mz = NSEN_1993::f_6_61_util_Mz(
            design_load.Mz,
            table6_7.delta_Mz_Ed,
            table6_7.Wz * self.mat.f_y(&LimitStateType::K),
            tableb_1.k_yz,
            self.mat.gamma_m1(&LimitStateType::D),
        );
        NSEN_1993::f_6_61(util_n, util_my, util_mz)
    }
    #[must_use]
    pub fn dc_6_62(
        &self,
        design_load: &LoadCase,
        c_my: f64,
        c_mz: f64,
        mu_cr: f64,
        lk: f64,
        buckle_curve_z: &BuckleCurve,
        ltb_curve: &LTBCurve,
    ) -> f64 {
        let khi_buckle_reduction_factor = self.khi(lk, &Axis::Z, buckle_curve_z);
        let table6_7 = Table6_7::from_crs_class(
            &self.crs,
            &self.cross_section_class(&CrossSectionClassCase::WebCompression),
        );
        let tableb_1 = TableB1::from_crs_class(
            self.euler_load(lk, &Axis::Y),
            self.euler_load(lk, &Axis::Z),
            c_my,
            c_mz,
            self,
            &self.cross_section_class(&CrossSectionClassCase::WebCompression),
            buckle_curve_z,
            design_load.N,
        );
        let util_n = NSEN_1993::f_6_62_util_N(
            design_load.N,
            khi_buckle_reduction_factor,
            self.N_pl(&LimitStateType::K),
            self.mat.gamma_m1(&LimitStateType::D),
        );
        let util_my = NSEN_1993::f_6_62_util_My(
            design_load.My,
            table6_7.delta_My_Ed,
            table6_7.Wy * self.mat.f_y(&LimitStateType::K),
            tableb_1.k_zy,
            self.khi_lt(lk, ltb_curve, mu_cr),
            self.mat.gamma_m1(&LimitStateType::D),
        );
        let util_mz = NSEN_1993::f_6_62_util_Mz(
            design_load.Mz,
            table6_7.delta_Mz_Ed,
            table6_7.Wz * self.mat.f_y(&LimitStateType::K),
            tableb_1.k_zz,
            self.mat.gamma_m1(&LimitStateType::D),
        );
        NSEN_1993::f_6_62(util_n, util_my, util_mz)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn N_pl(&self, limit_state_type: &LimitStateType) -> f64 {
        self.mat.f_y(limit_state_type) * self.crs.area()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn V_pl(&self, axis: &Axis, limit_state_type: &LimitStateType) -> f64 {
        self.mat.f_y(limit_state_type) * self.crs.area_shear(axis) / 3f64.sqrt()
    }

    #[must_use]
    pub fn buckle_cap(
        &self,
        lk: f64,
        axis: &Axis,
        buckle_curve: &NSEN_1993::BuckleCurve,
        limit_state_type: &LimitStateType,
    ) -> f64 {
        {
            // Eurocode 1993 buckling
            let khi = self.khi(lk, axis, buckle_curve);
            NSEN_1993::f_6_47(
                khi,
                self.crs.area(),
                self.mat.f_y(&LimitStateType::K),
                self.mat.gamma_m1(limit_state_type),
            )
        }
    }

    #[must_use]
    pub fn khi(&self, lk: f64, axis: &Axis, buckle_curve: &BuckleCurve) -> f64 {
        let lambda = self.lambda(self.euler_load(lk, axis));
        let phi = NSEN_1993::f_6_49_phi(buckle_curve.alpha(), lambda);
        NSEN_1993::f_6_49(phi, lambda)
    }
    #[must_use]
    pub fn khi_lt(&self, lk: f64, buckle_curve: &LTBCurve, mu_cr: f64) -> f64 {
        let lambda_lt = self.lambda_lt(self.M_cr(lk, mu_cr));
        let phi_lt = NSEN_1993::f_6_56_phi_LT(buckle_curve.alpha(), lambda_lt);
        NSEN_1993::f_6_56(phi_lt, lambda_lt)
    }
    #[must_use]
    pub fn lambda(&self, n_cr: f64) -> f64 {
        NSEN_1993::f_6_49_lambda(self.crs.area(), self.mat.f_y(&LimitStateType::K), n_cr)
    }
    #[must_use]
    pub fn lambda_lt(&self, m_cr: f64) -> f64 {
        let crs_class = self.cross_section_class(&CrossSectionClassCase::WebCompression);
        let wy = match crs_class {
            CrossSectionClass::One | CrossSectionClass::Two => self.crs.w_pl(&Axis::Y),
            CrossSectionClass::Three => self.crs.w_el(&Axis::Y),
            CrossSectionClass::Four => self.crs.w_eff(&Axis::Y),
        };
        NSEN_1993::f_6_56_lambda_LT(wy, self.mat.f_y(&LimitStateType::K), m_cr)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_el(&self, axis: &Axis, limit_state_type: &LimitStateType) -> f64 {
        self.crs.w_el(axis) * self.mat.f_y(limit_state_type)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_pl(&self, axis: &Axis, limit_state_type: &LimitStateType) -> f64 {
        self.crs.w_pl(axis) * self.mat.f_y(limit_state_type)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn EA(&self) -> f64 {
        self.mat.E() * self.crs.area()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn EI(&self, axis: &Axis) -> f64 {
        let I = self.crs.I(axis);
        I * self.mat.E()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn GI(&self, axis: &Axis) -> f64 {
        let I = self.crs.I(axis);
        I * self.mat.G()
    }
    #[must_use]
    pub fn euler_load(&self, lk: f64, axis: &Axis) -> f64 {
        self.EI(axis) * (std::f64::consts::PI / lk).powi(2)
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_0_cr(&self, length: f64) -> f64 {
        let pi = std::f64::consts::PI;
        (pi / length) * (self.GI(&Axis::X) * self.EI(&Axis::Z)).sqrt()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn M_cr(&self, length: f64, mu_cr: f64) -> f64 {
        self.M_0_cr(length) * mu_cr
    }
    #[must_use]
    pub fn self_weight_kg_per_meter(&self) -> f64 {
        let area_in_squaremillimeter = self.crs.area();
        let area_in_squaremeter = area_in_squaremillimeter / 1_000_000.0;
        // m^2 * kg/m^3 = kg/m
        area_in_squaremeter * self.mat.rho()
    }
    #[allow(non_snake_case)]
    #[must_use]
    pub fn self_weight_kN_per_meter(&self) -> f64 {
        // kg/m * m/s^2 / 1000 = kN/m
        self.self_weight_kg_per_meter() * crate::constants::G / 1000.0
    }
    #[must_use]
    pub fn json(&self) -> Value {
        match self.crs.variant() {
            crate::crs::Variant::HEB => {
                json!({
                    "EA": self.EA(),
                    "EI_y": self.EI(&Axis::Y),
                    "EI_z": self.EI(&Axis::Z),
                    "self_weight_kg_pr_meter": self.self_weight_kg_per_meter(),
                    "self_weight_kN_pr_meter": self.self_weight_kN_per_meter(),
                    "Cross_section_class_web_bending": self.crs.cross_section_class(self.mat.epsilon(), &CrossSectionClassCase::WebBending).to_num(),
                    "Cross_section_class_web_compression": self.crs.cross_section_class(self.mat.epsilon(), &CrossSectionClassCase::WebCompression).to_num(),
                    "Cross_section_class_flange_compression": self.crs.cross_section_class(self.mat.epsilon(), &CrossSectionClassCase::FlangeCompression).to_num(),
                    "N_pl_k": self.N_pl( &LimitStateType::K),
                    "V_pl_y_k": self.V_pl(&Axis::Y, &LimitStateType::K),
                    "M_el_y_k": self.M_el(&Axis::Y,&LimitStateType::K),
                    "M_pl_y_k": self.M_pl(&Axis::Y,&LimitStateType::K),
                    "V_pl_z_k": self.V_pl(&Axis::Z, &LimitStateType::K),
                    "M_el_z_k": self.M_el(&Axis::Z,&LimitStateType::K),
                    "M_pl_z_k": self.M_pl(&Axis::Z,&LimitStateType::K),
                    "N_pl_d": self.N_pl(&LimitStateType::D),
                    "V_pl_y_d": self.V_pl(&Axis::Y, &LimitStateType::D),
                    "M_el_y_d": self.M_el(&Axis::Y,&LimitStateType::D),
                    "M_pl_y_d": self.M_pl(&Axis::Y,&LimitStateType::D),
                    "V_pl_z_d": self.V_pl(&Axis::Z, &LimitStateType::D),
                    "M_el_z_d": self.M_el(&Axis::Z,&LimitStateType::D),
                    "M_pl_z_d": self.M_pl(&Axis::Z,&LimitStateType::D),
                })
            }
            crate::crs::Variant::CHS => {
                json!({
                    "EA": self.EA(),
                    "EI": self.EI(&Axis::Y),
                    "self_weight_kg_pr_meter": self.self_weight_kg_per_meter(),
                    "self_weight_kN_pr_meter": self.self_weight_kN_per_meter(),
                    "Cross_section_class": self.crs.cross_section_class(self.mat.epsilon(), &CrossSectionClassCase::None).to_num(),
                    "N_pl_k": self.N_pl( &LimitStateType::K),
                    "N_pl_d": self.N_pl(&LimitStateType::D),
                    "V_pl_k": self.V_pl(&Axis::Y, &LimitStateType::K),
                    "V_pl_d": self.V_pl(&Axis::Y, &LimitStateType::D),
                    "M_el_k": self.M_el(&Axis::Y,&LimitStateType::K),
                    "M_el_d": self.M_el(&Axis::Y,&LimitStateType::D),
                    "M_pl_k": self.M_pl(&Axis::Y,&LimitStateType::K),
                    "M_pl_d": self.M_pl(&Axis::Y,&LimitStateType::D),
                })
            }
        }
    }

    #[must_use]
    pub fn cross_section_class(&self, case: &CrossSectionClassCase) -> CrossSectionClass {
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
        assert_zeq!(mmb.M_el(&Axis::Y, &LimitStateType::K), 31_914_500.0);
    }

    #[test]
    fn ea() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EA(), 546_000_000.0);
    }
    #[test]
    fn ei_y() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI(&Axis::Y), 945_000_000_000.0);
    }
    #[test]
    fn ei_z() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.EI(&Axis::Z), 350_700_000_000.0);
    }
    #[test]
    fn gi() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.GI(&Axis::X), 7_503_461_538.461_537);
    }
    #[allow(non_snake_case)]
    #[test]
    fn m_0_cr() {
        let mmb = ColumnBeam::default();
        let length = 10000.0;
        assert_zeq!(mmb.M_0_cr(length), 16_115_678.172_546_148);
    }
    #[test]
    fn euler_load() {
        let mmb = ColumnBeam::default();
        let lk = 10_000.0;
        assert_zeq!(mmb.euler_load(lk, &Axis::Z), 34_612.702_634_620_38);
    }
    #[test]
    fn lambda() {
        let mmb = ColumnBeam::default();
        assert_zeq!(mmb.lambda(mmb.euler_load(10_000.0, &Axis::Z)), 5.163_962);
    }
    #[allow(non_snake_case)]
    #[test]
    fn lambda_lT() {
        let mmb = ColumnBeam::default();
        let mcr = 1.35 * mmb.M_0_cr(10_000.0);
        assert_zeq!(mmb.lambda_lt(mcr), 1.302_685);
    }
}
