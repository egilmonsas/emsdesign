#[allow(non_snake_case)]

pub fn f_6_2(N_ed: f64, My_ed: f64, Mz_ed: f64, N_rd: f64, My_rd: f64, Mz_rd: f64) -> f64 {
    N_ed / N_rd + My_ed / My_rd + Mz_ed / Mz_rd
}
pub fn f_6_47(khi: f64, area: f64, fy: f64, gamma_1: f64) -> f64 {
    khi * area * fy / gamma_1
}
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

pub fn _get_alpha(curve: BuckleCurve) -> f64 {
    match curve {
        BuckleCurve::A0 => 0.13,
        BuckleCurve::A => 0.21,
        BuckleCurve::B => 0.34,
        BuckleCurve::C => 0.49,
        BuckleCurve::D => 0.76,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn test_f_6_2() {
        let n_ed = 100.0;
        let my_ed = 100.0;
        let mz_ed = 100.0;
        let n_rd = 300.0;
        let my_rd = 300.0;
        let mz_rd = 300.0;

        let util = f_6_2(100.0, 100.0, 100.0, 300.0, 300.0, 300.0);
        assert_zeq!(util, 1.0);
    }
}
