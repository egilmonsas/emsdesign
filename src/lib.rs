#[macro_use]
pub mod zeq;
pub mod crs;
pub mod erc;
pub mod load;
pub mod mat;
pub mod mmb;

pub enum Axis {
    X,
    Y,
    Z,
}
impl Axis {
    pub fn get(identifier: &str) -> Self {
        match identifier {
            "X" => Axis::X,
            "Y" => Axis::Y,
            "Z" => Axis::Z,
            _ => Axis::Y,
        }
    }
}

pub enum Gamma {
    K,
    D,
}
impl Gamma {
    pub fn get(identifier: &str) -> Self {
        match identifier {
            "K" | "k" | "Characteristic" | "characteristic" => Gamma::K,
            "D" | "d" | "Design" | "design" => Gamma::D,
            _ => Gamma::K,
        }
    }
}
