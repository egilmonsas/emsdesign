#![warn(clippy::pedantic, clippy::nursery, clippy::cargo, clippy::unwrap_used)]
#[macro_use]
pub mod zequality;
pub mod crs;
pub mod erc;
pub mod err;
pub mod load;
pub mod mat;
pub mod mmb;

pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    #[must_use] pub fn get(identifier: &str) -> Self {
        match identifier {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => Self::Y,
        }
    }
}

pub enum Gamma {
    K,
    D,
}
impl Gamma {
    #[must_use] pub fn get(identifier: &str) -> Self {
        match identifier {
            "K" | "k" | "Characteristic" | "characteristic" => Self::K,
            "D" | "d" | "Design" | "design" => Self::D,
            _ => Self::K,
        }
    }
}
