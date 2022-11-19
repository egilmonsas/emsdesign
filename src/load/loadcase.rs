#[allow(non_snake_case)]
pub struct LoadCase {
    pub N: f64,
    pub Mx: f64,
    pub My: f64,
    pub Mz: f64,
}

impl LoadCase {
    pub fn get_all(&self) -> (f64, f64, f64, f64) {
        (self.N, self.Mx, self.My, self.Mz)
    }
}
