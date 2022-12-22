#[allow(non_snake_case)]

/// Design check 6.2 for columnbeam
pub fn f_6_2(N_ed: f64, My_ed: f64, Mz_ed: f64, N_rd: f64, My_rd: f64, Mz_rd: f64) -> f64 {
    N_ed / N_rd + My_ed / My_rd + Mz_ed / Mz_rd
}

/// Reduced capacity to account for shear forces
/// Not necessary to reduce if Ved < 0.5 Vrd
pub fn f_6_29(fy:f64, rho:f64)->f64{
    (1.0-rho)*fy
}

/// Reduction factor rho for shear when computing moment
pub fn _compute_rho(Ved:f64,Vpl_rd:f64)->f64{
    ((2.0*Ved)/(Vpl_rd)-1.0).powi(2)
}


/*----------------- BUCKLING ------------------*/

/// Buckling capacity for centric loaded column
pub fn f_6_47(khi: f64, area: f64, fy: f64, gamma_1: f64) -> f64 {
    khi * area * fy / gamma_1
}

/// Buckling reduction factor
pub fn f_6_49(phi: f64, lambda: f64) -> f64 {
    1.0 / (phi + (phi.powi(2) - lambda.powi(2)))
}

pub fn _compute_phi(alpha: f64, lambda: f64) -> f64 {
    0.5 * (1.0 + alpha * (lambda - 0.2) + lambda.powi(2))
}

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

impl BuckleCurve{
    pub fn alpha(&self)->f64{
        match self {
            BuckleCurve::A0 => 0.13,
            BuckleCurve::A => 0.21,
            BuckleCurve::B => 0.34,
            BuckleCurve::C => 0.49,
            BuckleCurve::D => 0.76,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn test_f_6_2() {
        let util = f_6_2(100.0, 100.0, 100.0, 300.0, 300.0, 300.0);
        assert_zeq!(util, 1.0);
    }

    
    #[test]
    fn test_buckle_curve_function() {
        assert_zeq!(BuckleCurve::A0.alpha(), 0.13);
    }
}
