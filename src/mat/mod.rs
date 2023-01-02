use crate::LimitStateType;
use serde_json::{json, Value};

pub mod steel;

pub trait Material {
    #[allow(non_snake_case)]
    fn E(&self) -> f64;
    fn rho(&self) -> f64;
    fn f_y(&self, limit_state_type: &LimitStateType) -> f64;
    fn f_u(&self, limit_state_type: &LimitStateType) -> f64;
    fn gamma_m0(&self, limit_state_type: &LimitStateType) -> f64;
    fn gamma_m1(&self, limit_state_type: &LimitStateType) -> f64;

    fn json(&self) -> Value {
        let jsonout = json!({
            "E": self.E(),
            "rho": self.rho(),
            "f_y":  self.f_y(&LimitStateType::K),
            "f_y_d":  self.f_y(&LimitStateType::D),
            "f_u": self.f_u(&LimitStateType::K),
            "f_u_d": self.f_u(&LimitStateType::D),
            "gamma_m0": self.gamma_m0(&LimitStateType::D),
            "gamma_m1": self.gamma_m1(&LimitStateType::D),
        });
        jsonout
    }
}
