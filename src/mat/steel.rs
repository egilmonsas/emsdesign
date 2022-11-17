#[allow(non_snake_case)]
pub struct Steel {
    pub E: f64,
    pub fy: f64,
}

impl Steel {
    pub fn default() -> Self {
        Self {
            E: 210_000.0,
            fy: 355.0,
        }
    }
}
