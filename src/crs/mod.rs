pub mod r#box;
pub mod circle;
pub mod rect;
pub mod standard;
pub mod tube;

pub trait CrossSection {
    /// Area in 
    /// [mm^2]
    fn area(&self) -> f64;
    /// Yc, Zc, as measured from bottom left corner in 
    /// [mm]
    fn centroid(&self) -> (f64, f64);
    #[allow(non_snake_case)]
    /// Inertia in [mm^4] about y-axis (usually defined as strong axis)
    fn Iy(&self) -> f64;
    #[allow(non_snake_case)]
    /// Inertia in [mm^4] about z-axis (usually defined as weak axis)
    fn Iz(&self) -> f64;
    /// Bending moment in [mm^3] about y-axis (usually defined as strong axis)
    fn wy(&self) -> f64;
    /// Bending moment in [mm^3] about z-axis (usually defined as weak axis)
    fn wz(&self) -> f64;
}
