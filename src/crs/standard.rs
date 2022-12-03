use super::CrossSection;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum IPE {
    IPE80,
    IPE100,
    IPE120,
    IPE140,
    IPE160,
    IPE180,
    IPE200,
    IPE220,
    IPE240,
    IPE270,
    IPE300,
    IPE330,
    IPE360,
    IPE400,
    IPE450,
    IPE500,
    IPE550,
    IPE600,
}
impl IPE {
    fn str(&self) -> &str {
        match self {
            IPE::IPE80 => "IPE 80",
            IPE::IPE100 => "IPE 100",
            IPE::IPE120 => "IPE 120",
            IPE::IPE140 => "IPE 140",
            IPE::IPE160 => "IPE 160",
            IPE::IPE180 => "IPE 180",
            IPE::IPE200 => "IPE 200",
            IPE::IPE220 => "IPE 220",
            IPE::IPE240 => "IPE 240",
            IPE::IPE270 => "IPE 270",
            IPE::IPE300 => "IPE 300",
            IPE::IPE330 => "IPE 330",
            IPE::IPE360 => "IPE 360",
            IPE::IPE400 => "IPE 400",
            IPE::IPE450 => "IPE 450",
            IPE::IPE500 => "IPE 500",
            IPE::IPE550 => "IPE 550",
            IPE::IPE600 => "IPE 600",
        }
    }
}
impl std::fmt::Display for IPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IPE::IPE80 => "IPE 80",
                IPE::IPE100 => "IPE 100",
                IPE::IPE120 => "IPE 120",
                IPE::IPE140 => "IPE 140",
                IPE::IPE160 => "IPE 160",
                IPE::IPE180 => "IPE 180",
                IPE::IPE200 => "IPE 200",
                IPE::IPE220 => "IPE 220",
                IPE::IPE240 => "IPE 240",
                IPE::IPE270 => "IPE 270",
                IPE::IPE300 => "IPE 300",
                IPE::IPE330 => "IPE 330",
                IPE::IPE360 => "IPE 360",
                IPE::IPE400 => "IPE 400",
                IPE::IPE450 => "IPE 450",
                IPE::IPE500 => "IPE 500",
                IPE::IPE550 => "IPE 550",
                IPE::IPE600 => "IPE 600",
            }
        )
    }
}
impl IPE {
    pub const ALL: [IPE; 18] = [
        IPE::IPE80,
        IPE::IPE100,
        IPE::IPE120,
        IPE::IPE140,
        IPE::IPE160,
        IPE::IPE180,
        IPE::IPE200,
        IPE::IPE220,
        IPE::IPE240,
        IPE::IPE270,
        IPE::IPE300,
        IPE::IPE330,
        IPE::IPE360,
        IPE::IPE400,
        IPE::IPE450,
        IPE::IPE500,
        IPE::IPE550,
        IPE::IPE600,
    ];
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Preset {
    h: f64,
    b: f64,
    a: f64,
    iy: f64,
    wy: f64,
    iz: f64,
    wz: f64,
}

impl Preset {
    pub fn new(ipe: &IPE) -> Self {
        let txt = std::fs::read_to_string(&"src\\crs\\presets\\IPE.ron").unwrap();
        let res: std::collections::HashMap<&str, Preset> = ron::from_str(txt.as_str()).unwrap();

        res[ipe.str()]
    }
}
impl Default for Preset {
    fn default() -> Self {
        Self {
            h: 200.0,
            b: 100.0,
            a: 2.85,
            iy: 19.4,
            wy: 194.0,
            iz: 1.42,
            wz: 28.50,
        }
    }
}
impl CrossSection for Preset {
    fn area(&self) -> f64 {
        self.a * 10.0f64.powi(3)
    }

    fn centroid(&self) -> (f64, f64) {
        (self.b / 2.0, self.h / 2.0)
    }
    #[allow(non_snake_case)]
    fn I(&self) -> (f64, f64) {
        (self.iy * 10.0f64.powi(6), self.iz * 10.0f64.powi(6))
    }
    fn w(&self) -> (f64, f64) {
        (self.wy * 10.0f64.powi(3), self.wz * 10.0f64.powi(3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn can_read() {
        let ipe = Preset::new(&IPE::IPE140);
        assert_zeq!(ipe.h, 140.0)
    }

    #[test]
    fn every_value_is_larger_than_the_smaller_crs() {
        let all_ipe = IPE::ALL;

        let mut prev_ipe = Preset::new(&all_ipe[0]);
        for ipe in all_ipe.iter() {
            let next_ipe = Preset::new(&ipe);

            assert!(next_ipe.h >= prev_ipe.h);
            assert!(next_ipe.b >= prev_ipe.b);
            assert!(next_ipe.a >= prev_ipe.a);
            assert!(next_ipe.iy >= prev_ipe.iy);
            assert!(next_ipe.wy >= prev_ipe.wy);
            assert!(next_ipe.iz >= prev_ipe.iz);
            assert!(next_ipe.wz >= prev_ipe.wz);

            let _ = std::mem::replace(&mut prev_ipe, next_ipe);
        }
    }

    #[test]
    fn area() {
        let ipe = Preset::new(&IPE::IPE200);
        assert_zeq!(ipe.area(), 2850.0);
    }

    #[test]
    fn centroid() {
        let ipe = Preset::new(&IPE::IPE200);
        let centroid = ipe.centroid();
        assert_zeq!(centroid.0, 50.0);
        assert_zeq!(centroid.1, 100.0);
    }

    #[test]
    fn second_moment_of_area() {
        let ipe = Preset::new(&IPE::IPE200);
        let centroid = ipe.I();
        assert_zeq!(centroid.0, 19_400_000.0);
        assert_zeq!(centroid.1, 1_420_000.0);
    }

    #[test]
    fn bending_moment() {
        let ipe = Preset::new(&IPE::IPE200);
        let centroid = ipe.w();
        assert_zeq!(centroid.0, 194_000.0);
        assert_zeq!(centroid.1, 28_500.0);
    }
}
