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
pub fn _compute_phi(alpha: f64, lambda: f64) -> f64 {
    #![allow(clippy::suboptimal_flops)]
    0.5 * (1.0 + alpha * (lambda - 0.2) + lambda.powi(2))
}

#[must_use]
pub fn _compute_lamba(area: f64, fy: f64, n_cr: f64) -> f64 {
    (area * fy / n_cr).sqrt()
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
        let res = _compute_lamba(a, fy, n_cr);
        assert_zeq!(res, 5.958_187);
    }
    #[test]
    fn test_phi_function() {
        let alpha = 0.2;
        let lambda = 2.0;
        let res = _compute_phi(alpha, lambda);
        assert_zeq!(res, 2.68);
    }
}
