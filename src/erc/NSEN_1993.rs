use crate::load::loadcase::LoadCase;
use crate::mmb::columnbeam::ColumnBeam;
#[allow(non_snake_case)]

pub fn f_6_2(load: LoadCase, mmb: ColumnBeam) -> f64 {
    let (N_ed, _, My_ed, Mz_ed) = load.get_all();
    let (N_rd, (My_rd, Mz_rd)) = (mmb.axial_cap(), mmb.moment_cap());

    N_ed / N_rd + My_ed / My_rd + Mz_ed / Mz_rd
}
