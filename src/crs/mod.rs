pub mod r#box;
pub mod circle;
pub mod rect;
pub mod standard;
pub mod tube;

pub trait CrossSection {
    fn area(&self) -> f64;
    fn centroid(&self) -> (f64, f64);
    #[allow(non_snake_case)]
    fn Iy(&self) -> f64;
    fn Iz(&self) -> f64;
    fn wy(&self) -> f64;
    fn wz(&self) -> f64;
}
