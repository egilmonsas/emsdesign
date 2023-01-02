use crate::LimitStateType;

use super::Material;

pub enum Variant {
    S235,
    S275,
    S355,
    S450,
}
impl Variant {
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "S235" => Some(Self::S235),
            "S275" => Some(Self::S275),
            "S355" => Some(Self::S355),
            "S450" => Some(Self::S450),
            _ => None,
        }
    }
    #[must_use]
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
    gamma_m0: f64,
    gamma_m1: f64,
}

impl Steel {
    #[must_use]
    pub const fn new(
        fy: f64,
        fu: f64,
        youngs_modulus: f64,
        density: f64,
        gamma_m0: f64,
        gamma_m1: f64,
    ) -> Self {
        Self {
            fy,
            fu,
            youngs_modulus,
            density,
            gamma_m0,
            gamma_m1,
        }
    }

    #[must_use]
    pub fn from(class: &Variant) -> Self {
        match class {
            Variant::S235 => Self {
                fy: 235.0,
                fu: 360.0,
                ..Default::default()
            },
            Variant::S275 => Self {
                fy: 275.0,
                fu: 430.0,
                ..Default::default()
            },
            Variant::S355 => Self {
                fy: 355.0,
                fu: 490.0,
                ..Default::default()
            },
            Variant::S450 => Self {
                fy: 440.0,
                fu: 550.0,
                ..Default::default()
            },
        }
    }
}

impl Default for Steel {
    fn default() -> Self {
        Self::new(355.0, 490.0, 210_000.0, 7850.0, 1.05, 1.05)
    }
}

impl Material for Steel {
    fn E(&self) -> f64 {
        self.youngs_modulus
    }
    fn rho(&self) -> f64 {
        self.density
    }
    fn f_y(&self, limit_state_type: &LimitStateType) -> f64 {
        self.fy / self.gamma_m0(limit_state_type)
    }
    fn f_u(&self, limit_state_type: &LimitStateType) -> f64 {
        self.fu / self.gamma_m0(limit_state_type)
    }
    fn gamma_m0(&self, limit_state_type: &LimitStateType) -> f64 {
        match limit_state_type {
            LimitStateType::K => 1.00,
            LimitStateType::D => self.gamma_m0,
        }
    }
    fn gamma_m1(&self, limit_state_type: &LimitStateType) -> f64 {
        match limit_state_type {
            LimitStateType::K => 1.00,
            LimitStateType::D => self.gamma_m1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn correct_gamma() {
        let steel = Steel::default();
        assert_zeq!(steel.gamma_m0(&LimitStateType::K), 1.00);
        assert_zeq!(steel.gamma_m0(&LimitStateType::D), 1.05);
    }
    #[test]
    fn can_create_expected_steel_class() {
        let steel = Steel::from(&Variant::S355);
        assert_zeq!(steel.f_y(&LimitStateType::K), 355.0);
        assert_zeq!(steel.f_u(&LimitStateType::K), 490.0);
    }
}
