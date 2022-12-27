use emsdesign::crs::rect::CrsRect;
use emsdesign::erc::NSEN_1993::*;
use emsdesign::load::loadcase::LoadCase;
use emsdesign::mat::steel::Steel;
use emsdesign::mmb::columnbeam::ColumnBeam;
use emsdesign::{Axis, Gamma};

fn main() {
    let cmb = ColumnBeam::new(Box::new(CrsRect::new(100.0, 200.0)), Steel::default());

    let load = LoadCase::new().axial_kN(500.0);

    let utilization: f64 = {
        let (n_ed, _, my_ed, mz_ed) = load.get_all();

        let n_rd = cmb.buckle_cap(5000.0, Axis::Z, &Gamma::K);

        f_6_2(
            n_ed,
            my_ed,
            mz_ed,
            n_rd,
            cmb.M_pl(Axis::Y, &Gamma::K),
            cmb.M_pl(Axis::Z, &Gamma::K),
        )
    };

    println!("{}", load);
    println!("Design check: {:.0}%", utilization * 100.0);
}
