pub mod r#box;
pub mod circle;
pub mod rect;
pub mod standard;
pub mod tube;

use serde_json::{json, Value};

pub trait CrossSection {
    /// Width of bounding box (along y-axis) in
    /// [mm]
    fn width(&self) -> f64;
    /// height of bounding box (along z-axis) in
    /// [mm]
    fn height(&self) -> f64;
    /// Area in
    /// [mm^2]
    fn area(&self) -> f64;
    /// Yc, Zc, as measured from bottom left corner in
    /// [mm]
    fn centroid(&self) -> (f64, f64) {
        (self.width() / 2.0, self.height() / 2.0)
    }
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
    /// Bending moment in [mm^3] about y-axis (usually defined as strong axis)
    fn wy_pl(&self) -> f64;
    /// Bending moment in [mm^3] about z-axis (usually defined as weak axis)
    fn wz_pl(&self) -> f64;

    fn json(&self) -> Value {
        let jsonout = json!({
            "width": self.width(),
            "height": self.height(),
            "area":  self.area(),
            "Iy": self.Iy(),
            "Iz": self.Iz(),
            "wy": self.wy(),
            "wy,pl": self.wy_pl(),
            "wz": self.wz(),
            "wz,pl": self.wz_pl(),
        });
        jsonout
    }
}
