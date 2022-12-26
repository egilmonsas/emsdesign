pub mod r#box;
pub mod circle;
pub mod rect;
pub mod standard;
pub mod tube;

use serde_json::{json, Value};

use crate::Axis;

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
    /// Inertia in [mm^4] about a given axis
    fn I(&self, axis: Axis) -> f64;
    #[allow(non_snake_case)]
    /// Inertia in [mm^4] about a given axis
    fn w_el(&self, axis: Axis) -> f64;
    /// Bending moment in [mm^3] about a given axis
    fn w_pl(&self, axis: Axis) -> f64;

    fn json(&self) -> Value {
        let jsonout = json!({
            "width": self.width(),
            "height": self.height(),
            "area":  self.area(),
            "I_y": self.I(Axis::Y),
            "I_z": self.I(Axis::Z),
            "w_el_y": self.w_el(Axis::Y),
            "w_pl_y": self.w_pl(Axis::Y),
            "w_el_z": self.w_el(Axis::Z),
            "w_pl_z": self.w_pl(Axis::Z),
        });
        jsonout
    }
}
