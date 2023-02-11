use crate::{
    crs::{CrossSection, CrossSectionClass, Variant},
    mat::Material,
    mmb::columnbeam::ColumnBeam,
    Axis,
};

#[allow(non_snake_case)]

/// Design check 6.2 for columnbeam
#[must_use]
pub fn f_6_2(N_ed: f64, My_ed: f64, Mz_ed: f64, N_rd: f64, My_rd: f64, Mz_rd: f64) -> f64 {
    N_ed / N_rd + My_ed / My_rd + Mz_ed / Mz_rd
}

/// Reduced capacity to account for shear forces
/// Not necessary to reduce if Ved < 0.5 Vrd
#[must_use]
pub fn f_6_29(fy: f64, rho: f64) -> f64 {
    (1.0 - rho) * fy
}

/// Reduction factor rho for shear when computing moment
#[must_use]
pub fn _compute_rho(Ved: f64, Vpl_rd: f64) -> f64 {
    ((2.0 * Ved) / (Vpl_rd) - 1.0).powi(2)
}

/*----------------- BUCKLING ------------------*/

/// Buckling capacity for centric loaded column
#[must_use]
pub fn f_6_47(khi: f64, area: f64, fy: f64, gamma_1: f64) -> f64 {
    khi * area * fy / gamma_1
}

/// Buckling reduction factor
#[must_use]
pub fn f_6_49(phi: f64, lambda: f64) -> f64 {
    // Calculate khi
    #![allow(clippy::suboptimal_flops)]
    let khi_reduction_factor = 1.0 / (phi + (phi.powi(2) - lambda.powi(2)).sqrt());
    // Upper bounded by 1.0
    khi_reduction_factor.clamp(0.0, 1.0)
}
#[must_use]
pub fn f_6_49_phi(alpha: f64, lambda: f64) -> f64 {
    #![allow(clippy::suboptimal_flops)]
    0.5 * (1.0 + alpha * (lambda - 0.2) + lambda.powi(2))
}

#[must_use]
pub fn f_6_49_lambda(area: f64, fy: f64, n_cr: f64) -> f64 {
    (area * fy / n_cr).sqrt()
}
/// Buckling reduction factor
#[must_use]
pub fn f_6_56(phi_LT: f64, lambda_LT: f64) -> f64 {
    // Calculate khi
    let khi_reduction_factor = 1.0 / (phi_LT + (phi_LT.powi(2) - lambda_LT.powi(2)).sqrt());
    // Upper bounded by 1.0
    khi_reduction_factor.clamp(0.0, 1.0)
}
#[must_use]
pub fn f_6_56_phi_LT(alpha_LT: f64, lambda_LT: f64) -> f64 {
    0.5 * (1.0 + alpha_LT * (lambda_LT - 0.2) + lambda_LT.powi(2))
}

#[must_use]
pub fn f_6_56_lambda_LT(Wy: f64, fy: f64, M_cr: f64) -> f64 {
    (Wy * fy / M_cr).sqrt()
}
#[must_use]
pub fn f_6_61(util_N: f64, util_My: f64, util_Mz: f64) -> f64 {
    util_N + util_My + util_Mz
}
#[must_use]
pub fn f_6_61_util_N(N_ed: f64, ksi_y: f64, N_rk: f64, gamma_1: f64) -> f64 {
    N_ed / (ksi_y * N_rk / gamma_1)
}
#[must_use]
pub fn f_6_61_util_My(
    My_ed: f64,
    delta_My_ed: f64,
    My_rk: f64,
    k_yy: f64,
    khi_LT: f64,
    gamma_1: f64,
) -> f64 {
    k_yy * ((My_ed + delta_My_ed) / (khi_LT * (My_rk / gamma_1)))
}
#[must_use]
pub fn f_6_61_util_Mz(Mz_ed: f64, delta_Mz_ed: f64, Mz_rk: f64, k_yz: f64, gamma_1: f64) -> f64 {
    k_yz * ((Mz_ed + delta_Mz_ed) / (Mz_rk / gamma_1))
}
#[must_use]
pub fn f_6_62(util_N: f64, util_My: f64, util_Mz: f64) -> f64 {
    util_N + util_My + util_Mz
}
#[must_use]
pub fn f_6_62_util_N(N_ed: f64, ksi_z: f64, N_rk: f64, gamma_1: f64) -> f64 {
    N_ed / (ksi_z * N_rk / gamma_1)
}
#[must_use]
pub fn f_6_62_util_My(
    My_ed: f64,
    delta_My_ed: f64,
    My_rk: f64,
    k_zy: f64,
    khi_LT: f64,
    gamma_1: f64,
) -> f64 {
    k_zy * ((My_ed + delta_My_ed) / (khi_LT * (My_rk / gamma_1)))
}
#[must_use]
pub fn f_6_62_util_Mz(Mz_ed: f64, delta_Mz_ed: f64, Mz_rk: f64, k_zz: f64, gamma_1: f64) -> f64 {
    k_zz * ((Mz_ed + delta_Mz_ed) / (Mz_rk / gamma_1))
}
pub struct Table6_7 {
    pub Ai: f64,
    pub Wy: f64,
    pub Wz: f64,
    pub delta_My_Ed: f64,
    pub delta_Mz_Ed: f64,
}

impl Table6_7 {
    #[must_use]
    pub fn from_crs_class(crs: &Box<dyn CrossSection>, crs_class: &CrossSectionClass) -> Self {
        match crs_class {
            CrossSectionClass::One | CrossSectionClass::Two => Self {
                Ai: crs.area(),
                Wy: crs.w_pl(&Axis::Y),
                Wz: crs.w_pl(&Axis::Z),
                delta_My_Ed: 0.0,
                delta_Mz_Ed: 0.0,
            },
            CrossSectionClass::Three => Self {
                Ai: crs.area(),
                Wy: crs.w_el(&Axis::Y),
                Wz: crs.w_el(&Axis::Z),
                delta_My_Ed: 0.0,
                delta_Mz_Ed: 0.0,
            },
            CrossSectionClass::Four => todo!(),
        }
    }
}
pub struct TableB_1 {
    pub k_yy: f64,
    pub k_yz: f64,
    pub k_zy: f64,
    pub k_zz: f64,
}
#[allow(clippy::suboptimal_flops, clippy::similar_names)]
impl TableB_1 {
    #[must_use]
    pub fn from_crs_class(
        n_cr_y: f64,
        n_cr_z: f64,
        c_my: f64,
        c_mz: f64,
        mmb: &ColumnBeam,
        crs_class: &CrossSectionClass,
        buckle_curve: &BuckleCurve,
        N_ed: f64,
        lk: f64,
    ) -> Self {
        let variant = mmb.crs.variant();
        let lambda_y = mmb.lambda(n_cr_y);
        let lambda_z = mmb.lambda(n_cr_z);
        let phi_y = f_6_49_phi(buckle_curve.alpha(), lambda_y);
        let phi_z = f_6_49_phi(buckle_curve.alpha(), lambda_z);
        let ksi_y = f_6_49(phi_y, lambda_y);
        let ksi_z = f_6_49(phi_z, lambda_z);
        let N_rk = mmb.N_pl(&crate::LimitStateType::K);
        let gamma_1 = mmb.mat.gamma_m1(&crate::LimitStateType::D);
        let util_y = N_ed / (ksi_y * N_rk / gamma_1);
        let util_z = N_ed / (ksi_z * N_rk / gamma_1);
        match crs_class {
            CrossSectionClass::Three | CrossSectionClass::Four => {
                let k_yy = (c_my * (1.0 + 0.6 * lambda_y * util_y))
                    .clamp(0.0, c_my * (1.0 + 0.6 * util_y));
                let k_zz = (c_mz * (1.0 + 0.6 * lambda_z * util_z))
                    .clamp(0.0, c_mz * (1.0 + 0.6 * util_z));
                let k_yz = k_zz;
                let k_zy = 0.8 * k_yy;
                Self {
                    k_yy,
                    k_yz,
                    k_zy,
                    k_zz,
                }
            }
            CrossSectionClass::One | CrossSectionClass::Two => {
                let k_yy = (c_my * (1.0 + (lambda_y - 0.2) * util_y))
                    .clamp(0.0, c_my * (1.0 + 0.8 * util_y));

                let k_zz = match variant {
                    Variant::HEB => (c_mz * (1.0 + (2.0 * lambda_z - 0.6) * util_z))
                        .clamp(0.0, c_mz * (1.0 + 1.4 * util_z)),
                    Variant::CHS => (c_mz * (1.0 + (lambda_z - 0.2) * util_z))
                        .clamp(0.0, c_mz * (1.0 + 0.8 * util_z)),
                };

                let k_yz = 0.6 * k_zz;
                let k_zy = 0.6 * k_yy;
                Self {
                    k_yy,
                    k_yz,
                    k_zy,
                    k_zz,
                }
            }
        }
    }
}

pub enum BuckleCurve {
    A0,
    A,
    B,
    C,
    D,
}

impl BuckleCurve {
    #[must_use]
    pub const fn alpha(&self) -> f64 {
        match self {
            Self::A0 => 0.13,
            Self::A => 0.21,
            Self::B => 0.34,
            Self::C => 0.49,
            Self::D => 0.76,
        }
    }
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "A0" | "a0" => Some(Self::A0),
            "A" | "a" => Some(Self::A),
            "B" | "b" => Some(Self::B),
            "C" | "c" => Some(Self::C),
            "D" | "d" => Some(Self::D),
            _ => None,
        }
    }
}
pub enum LTBCurve {
    A,
    B,
    C,
    D,
}

impl LTBCurve {
    #[must_use]
    pub const fn alpha(&self) -> f64 {
        match self {
            Self::A => 0.21,
            Self::B => 0.34,
            Self::C => 0.49,
            Self::D => 0.76,
        }
    }
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "A" | "a" => Some(Self::A),
            "B" | "b" => Some(Self::B),
            "C" | "c" => Some(Self::C),
            "D" | "d" => Some(Self::D),
            _ => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn test_f_6_2() {
        let util = f_6_2(100.0, 100.0, 100.0, 300.0, 300.0, 300.0);
        assert_zeq!(util, 1.0);
    }
    #[test]
    pub fn test_f_6_47() {
        let khi = 0.6;
        let area = 1000.0;
        let fy = 355.0;
        let gamma_1 = 1.1;
        let res = f_6_47(khi, area, fy, gamma_1);
        assert_zeq!(res, 193_636.363_636_363_62);
    }
    #[test]
    pub fn function_f_6_49_yields_correct_value_for_normal_input() {
        assert_zeq!(f_6_49(2.2, 2.0), 0.320_871);
    }
    #[test]
    pub fn function_f_6_49_clamps_result_correctly() {
        assert_zeq!(f_6_49(0.2, 0.0), 1.0);
    }

    #[test]
    fn test_buckle_curve_function() {
        assert_zeq!(BuckleCurve::A0.alpha(), 0.13);
    }

    #[test]
    fn test_lambda_function() {
        let a = 1000.0;
        let fy = 355.0;
        let n_cr = 10000.0;
        let res = f_6_49_lambda(a, fy, n_cr);
        assert_zeq!(res, 5.958_187);
    }
    #[test]
    fn test_phi_function() {
        let alpha = 0.2;
        let lambda = 2.0;
        let res = f_6_49_phi(alpha, lambda);
        assert_zeq!(res, 2.68);
    }
}
