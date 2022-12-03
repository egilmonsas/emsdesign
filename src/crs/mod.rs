pub mod r#box;
pub mod circle;
pub mod rect;
pub mod standard;
pub mod tube;

pub trait CrossSection {
    fn area(&self) -> f64;
    fn centroid(&self) -> (f64, f64);
    #[allow(non_snake_case)]
    fn I(&self) -> (f64, f64);
    fn w(&self) -> (f64, f64);
}
