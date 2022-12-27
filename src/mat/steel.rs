use crate::Gamma;

use super::Material;

pub enum SteelVariant {
    S235,
    S275,
    S355,
    S450,
}
impl SteelVariant {
    pub fn get(identifier: &str) -> Self {
        return match identifier {
            "S235" => SteelVariant::S235,
            "S275" => SteelVariant::S275,
            "S355" => SteelVariant::S450,
            "S450" => SteelVariant::S355,
            _ => SteelVariant::S355,
        };
    }
    pub fn variants() -> Vec<String> {
        vec![
            String::from("S235"),
            String::from("S275"),
            String::from("S355"),
            String::from("S450"),
        ]
    }
}

#[allow(non_snake_case)]
pub struct Steel {
    fy: f64,
    fu: f64,
    youngs_modulus: f64,
    density: f64,
    gamma_m_0: f64,
}

impl Steel {
    pub fn new(fy: f64, fu: f64, youngs_modulus: f64, density: f64, gamma_m_0: f64) -> Self {
        Self {
            fy,
            fu,
            youngs_modulus,
            density,
            gamma_m_0,
        }
    }

    pub fn from(class: SteelVariant) -> Self {
        match class {
            SteelVariant::S235 => Self {
                fy: 235.0,
                fu: 360.0,
                ..Default::default()
            },
            SteelVariant::S275 => Self {
                fy: 275.0,
                fu: 430.0,
                ..Default::default()
            },
            SteelVariant::S355 => Self {
                fy: 355.0,
                fu: 490.0,
                ..Default::default()
            },
            SteelVariant::S450 => Self {
                fy: 440.0,
                fu: 550.0,
                ..Default::default()
            },
        }
    }
}

impl Default for Steel {
    fn default() -> Self {
        Self::new(355.0, 490.0, 210000.0, 7850.0, 1.05)
    }
}

impl Material for Steel {
    fn E(&self) -> f64 {
        self.youngs_modulus
    }
    fn rho(&self) -> f64 {
        self.density
    }
    fn f_y(&self, limit_state_type: &Gamma) -> f64 {
        self.fy / self.gamma(limit_state_type)
    }
    fn f_u(&self, limit_state_type: &Gamma) -> f64 {
        self.fu / self.gamma(limit_state_type)
    }
    fn gamma(&self, limit_state_type: &Gamma) -> f64 {
        match limit_state_type {
            Gamma::K => 1.00,
            Gamma::D => self.gamma_m_0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn correct_gamma() {
        let steel = Steel::default();
        assert_zeq!(steel.gamma(&Gamma::K), 1.00);
        assert_zeq!(steel.gamma(&Gamma::D), 1.05);
    }
    #[test]
    fn can_create_expected_steel_class() {
        let steel = Steel::from(SteelVariant::S355);
        assert_zeq!(steel.f_y(&Gamma::K), 355.0);
        assert_zeq!(steel.f_u(&Gamma::K), 490.0);
    }
}
