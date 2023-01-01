#[allow(non_snake_case)]
pub struct LoadCase {
    pub N: f64,
    pub Mx: f64,
    pub My: f64,
    pub Mz: f64,
}

impl Default for LoadCase {
    fn default() -> Self {
        Self {
            N: 0.0,
            Mx: 0.0,
            My: 0.0,
            Mz: 0.0,
        }
    }
}

#[allow(non_snake_case)]
impl LoadCase {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn axial_kN(mut self, load: f64) -> Self {
        self.N = load * 1000.0;
        self
    }
    #[must_use]
    pub fn moment_kNm(mut self, mx: f64, my: f64, mz: f64) -> Self {
        self.Mx = mx * 1_000_000.0;
        self.My = my * 1_000_000.0;
        self.Mz = mz * 1_000_000.0;
        self
    }

    #[must_use]
    pub const fn get_all(&self) -> (f64, f64, f64, f64) {
        (self.N, self.Mx, self.My, self.Mz)
    }
}

impl std::fmt::Display for LoadCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Load case:
{: >10.1} kN 
{: >10.1} kNm
{: >10.1} kNm 
{: >10.1} kNm",
            self.N / 1_000.0,
            self.Mx / 1_000_000.0,
            self.My / 1_000_000.0,
            self.Mz / 1_000_000.0
        )
    }
}
