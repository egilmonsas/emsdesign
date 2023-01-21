#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, clippy::unwrap_used)]
#[macro_use]
pub mod zequality;
pub mod constants;
pub mod crs;
pub mod erc;
pub mod err;
pub mod load;
pub mod mat;
pub mod mmb;
pub mod test;
pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "X" => Some(Self::X),
            "Y" => Some(Self::Y),
            "Z" => Some(Self::Z),
            _ => None,
        }
    }
}

pub enum LimitStateType {
    K,
    D,
}
impl LimitStateType {
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "K" | "k" | "Characteristic" | "characteristic" => Some(Self::K),
            "D" | "d" | "Design" | "design" => Some(Self::D),
            _ => None,
        }
    }
}
