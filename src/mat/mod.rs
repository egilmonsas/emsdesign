use crate::Gamma;
use serde_json::{json, Value};

pub mod steel;

pub trait Material {
    #[allow(non_snake_case)]
    fn E(&self) -> f64;
    fn rho(&self) -> f64;
    fn f_y(&self, limit_state_type: &Gamma) -> f64;
    fn f_u(&self, limit_state_type: &Gamma) -> f64;
    fn gamma(&self, limit_state_type: &Gamma) -> f64;

    fn json(&self) -> Value {
        let jsonout = json!({
            "E": self.E(),
            "rho": self.rho(),
            "f_y":  self.f_y(&Gamma::K),
            "f_y_d":  self.f_y(&Gamma::D),
            "f_u": self.f_u(&Gamma::K),
            "f_u_d": self.f_u(&Gamma::D),
            "gamma_d": self.gamma(&Gamma::D),

        });
        jsonout
    }
}
