pub mod chs;
pub mod heb;

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
    /// Area in
    /// [mm^2]
    fn area_shear(&self, axis: Axis) -> f64;
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
            "A_v_y":  self.area_shear(Axis::Y),
            "A_v_z":  self.area_shear(Axis::Z),
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

use crate::crs::chs::*;
use crate::crs::heb::*;
use crate::err::EmsError;

#[derive(Clone, Copy)]
pub enum PRESETS {
    HEB,
    CHS,
}

impl PRESETS {
    #[must_use]
    pub fn get(identifier: &str) -> Option<Self> {
        match identifier {
            "HEB" => Some(Self::HEB),
            "CHS" => Some(Self::CHS),
            _ => None,
        }
    }
}

pub struct CrsLib {}

impl CrsLib {
    pub fn sections(preset: &PRESETS) -> Vec<&str> {
        match preset {
            PRESETS::HEB => HEBLIB.keys().cloned().collect(),
            PRESETS::CHS => CHSLIB.keys().cloned().collect(),
        }
    }
    pub fn get(preset: &PRESETS, key: &str) -> Box<dyn CrossSection> {
        match preset {
            PRESETS::HEB => Box::new(HEBLIB.get(key).cloned().unwrap_or_default()),
            PRESETS::CHS => Box::new(CHSLIB.get(key).cloned().unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn can_collect_vector_from_section_names() {
        let res = CrsLib::sections(&PRESETS::CHS);
        dbg!(res);
    }

    #[test]
    fn can_get_heb_beam() {
        let heb100 = CrsLib::get(&PRESETS::HEB, "HEB 100");
        dbg!(heb100.height());
        let heb400 = CrsLib::get(&PRESETS::HEB, "HEB 400");
        dbg!(heb400.height());
    }
}
