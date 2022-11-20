use emsdesign::crs::rect::CrsRect;
use emsdesign::erc::NSEN_1993::*;
use emsdesign::load::loadcase::LoadCase;
use emsdesign::mat::steel::Steel;
use emsdesign::mmb::columnbeam::ColumnBeam;

fn main() {
    let cmb = ColumnBeam::new(CrsRect::new(100.0, 200.0), Steel::default(), 5_000.0);

    let load = LoadCase::new().axial_kN(500.0);

    let utilization: f64 = {
        let (n_ed, _, my_ed, mz_ed) = load.get_all();

        let n_rd = {
            let gamma_1 = 1.15;
            let alpha = _get_alpha(BuckleCurve::C);

            let eulerloads = cmb.euler_load((5000.0, 1000.0));
            let ncr = eulerloads.0.min(eulerloads.1);
            let lambda = _compute_lamba(cmb.crs.area(), cmb.mat.fy, ncr);
            let phi = _compute_phi(alpha, lambda);
            let khi = f_6_49(phi, lambda);

            f_6_47(khi, cmb.crs.area(), cmb.mat.fy, gamma_1)
        };

        let (my_rd, mz_rd) = cmb.moment_cap();

        f_6_2(n_ed, my_ed, mz_ed, n_rd, my_rd, mz_rd)
    };

    println!("{}", load);
    println!("Design check: {:.0}%", utilization * 100.0);
}
