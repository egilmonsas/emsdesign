use super::{CrossSection, CrossSectionClass, CrossSectionClassCase};
use crate::Axis;
use phf::phf_ordered_map;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct CrsHEB {
    width: f64,
    height: f64,
    thickness_web: f64,
    thickness_flange: f64,
    radius: f64,
    area: f64,
    area_shear_y: f64,
    w_elastic_y: f64,
    w_plastic_y: f64,
    inertia_y: f64,
    area_shear_z: f64,
    w_elastic_z: f64,
    w_plastic_z: f64,
    inertia_z: f64,
}

impl CrsHEB {
    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        HEBLIB.get(key).cloned()
    }

    fn cclass_web_bending(&self, epsilon: f64) -> CrossSectionClass {
        // Tverrsnittsdeler som utsettes for bøyning
        #[allow(clippy::suboptimal_flops)]
        let c = self.height() - 2.0 * (self.thickness_flange - self.radius);
        match c / self.thickness_web {
            res if res <= 72.0 * epsilon => CrossSectionClass::One,
            res if res <= 83.0 * epsilon => CrossSectionClass::Two,
            res if res <= 124.0 * epsilon => CrossSectionClass::Three,
            _ => CrossSectionClass::Four,
        }
    }
    fn cclass_web_compression(&self, epsilon: f64) -> CrossSectionClass {
        // Tverrsnittsdeler som utsettes for rent trykk
        #[allow(clippy::suboptimal_flops)]
        let c = self.height() - 2.0 * (self.thickness_flange - self.radius);
        match c / self.thickness_web {
            res if res <= 33.0 * epsilon => CrossSectionClass::One,
            res if res <= 38.0 * epsilon => CrossSectionClass::Two,
            res if res <= 42.0 * epsilon => CrossSectionClass::Three,
            _ => CrossSectionClass::Four,
        }
    }
    fn cclass_flange_compression(&self, epsilon: f64) -> CrossSectionClass {
        let c = (self.width() - self.thickness_web) / 2.0 - self.radius;
        match c / self.thickness_flange {
            res if res <= 9.0 * epsilon => CrossSectionClass::One,
            res if res <= 10.0 * epsilon => CrossSectionClass::Two,
            res if res <= 14.0 * epsilon => CrossSectionClass::Three,
            _ => CrossSectionClass::Four,
        }
    }
}

impl Default for CrsHEB {
    fn default() -> Self {
        Self::from_key("HEB 100").expect("Could not extract section 'HEB 100'")
    }
}
impl CrossSection for CrsHEB {
    fn width(&self) -> f64 {
        self.width
    }
    fn height(&self) -> f64 {
        self.height
    }
    fn area(&self) -> f64 {
        self.area
    }

    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.inertia_y,
            Axis::Z => self.inertia_z,
        }
    }
    fn w_el(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.w_elastic_y,
            Axis::Z => self.w_elastic_z,
        }
    }
    fn w_pl(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.w_plastic_y,
            Axis::Z => self.w_plastic_z,
        }
    }

    fn area_shear(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y => self.area_shear_y,
            Axis::Z => self.area_shear_z,
        }
    }

    fn cross_section_class(&self, epsilon: f64, case: CrossSectionClassCase) -> CrossSectionClass {
        match case {
            CrossSectionClassCase::WebBending => self.cclass_web_bending(epsilon),
            CrossSectionClassCase::WebCompression => self.cclass_web_compression(epsilon),
            CrossSectionClassCase::WebBendingAndCompression => {
                todo!()
            }
            CrossSectionClassCase::FlangeCompression => self.cclass_flange_compression(epsilon),
            CrossSectionClassCase::FlangeBendingAndCompressionAtFreeEnd => {
                todo!()
            }
            CrossSectionClassCase::FlangeBendingAndTesionAtFreeEnd => {
                todo!()
            }
            CrossSectionClassCase::None => {
                todo!()
            }
        }
    }

    fn variant(&self) -> super::Variant {
        super::Variant::HEB
    }

    fn json(&self) -> serde_json::Value {
        let jsonout = json!({
            "width": self.width,
            "height": self.height,
            "area":  self.area,
            "thickness_flange": self.thickness_flange,
            "thickness_web": self.thickness_web,
            "radius1" :self.radius,
            "A_v_y": self.area_shear_y,
            "A_v_z": self.area_shear_z,
            "w_el_y": self.w_elastic_y,
            "w_pl_y": self.w_plastic_y,
            "w_el_z": self.w_elastic_z,
            "w_pl_z": self.w_plastic_z,
            "I_y": self.inertia_y,
            "I_z": self.inertia_z,
        });
        jsonout
    }
}
#[allow(clippy::unreadable_literal)]
pub static HEBLIB: phf::OrderedMap<&'static str, CrsHEB> = phf_ordered_map! {
    "HEB 100" => CrsHEB{width: 100.0_f64, height: 100.0_f64, area: 2600.0_f64, thickness_web: 6.0_f64, thickness_flange: 10.0_f64, radius: 12.0_f64, area_shear_y: 2000.0_f64, w_elastic_y: 89900.0_f64, w_plastic_y: 104000.0_f64, inertia_y: 4500000.0_f64, area_shear_z: 540.0_f64, w_elastic_z: 33500.0_f64, w_plastic_z: 51000.0_f64, inertia_z: 1670000.0_f64},
    "HEB 120" => CrsHEB{width: 120.0_f64, height: 120.0_f64, area: 3400.0_f64, thickness_web: 6.5_f64, thickness_flange: 11.0_f64, radius: 12.0_f64, area_shear_y: 2640.0_f64, w_elastic_y: 144000.0_f64, w_plastic_y: 165000.0_f64, inertia_y: 8640000.0_f64, area_shear_z: 708.0_f64, w_elastic_z: 52900.0_f64, w_plastic_z: 81000.0_f64, inertia_z: 3180000.0_f64},
    "HEB 140" => CrsHEB{width: 140.0_f64, height: 140.0_f64, area: 4300.0_f64, thickness_web: 7.0_f64, thickness_flange: 12.0_f64, radius: 12.0_f64, area_shear_y: 3360.0_f64, w_elastic_y: 216000.0_f64, w_plastic_y: 246000.0_f64, inertia_y: 15100000.0_f64, area_shear_z: 896.0000000000001_f64, w_elastic_z: 78500.0_f64, w_plastic_z: 120000.0_f64, inertia_z: 5500000.0_f64},
    "HEB 160" => CrsHEB{width: 160.0_f64, height: 160.0_f64, area: 5430.0_f64, thickness_web: 8.0_f64, thickness_flange: 13.0_f64, radius: 15.0_f64, area_shear_y: 4160.0_f64, w_elastic_y: 311000.0_f64, w_plastic_y: 354000.0_f64, inertia_y: 24900000.0_f64, area_shear_z: 1176.0_f64, w_elastic_z: 111000.0_f64, w_plastic_z: 170000.0_f64, inertia_z: 8890000.0_f64},
    "HEB 180" => CrsHEB{width: 180.0_f64, height: 180.0_f64, area: 6530.0_f64, thickness_web: 8.5_f64, thickness_flange: 14.0_f64, radius: 15.0_f64, area_shear_y: 5040.0_f64, w_elastic_y: 426000.0_f64, w_plastic_y: 482000.0_f64, inertia_y: 38300000.0_f64, area_shear_z: 1411.0_f64, w_elastic_z: 151000.0_f64, w_plastic_z: 231000.0_f64, inertia_z: 13600000.0_f64},
    "HEB 200" => CrsHEB{width: 200.0_f64, height: 200.0_f64, area: 7809.999999999999_f64, thickness_web: 9.0_f64, thickness_flange: 15.0_f64, radius: 18.0_f64, area_shear_y: 6000.0_f64, w_elastic_y: 570000.0_f64, w_plastic_y: 642000.0_f64, inertia_y: 57000000.0_f64, area_shear_z: 1664.9999999999998_f64, w_elastic_z: 200000.0_f64, w_plastic_z: 306000.0_f64, inertia_z: 20000000.0_f64},
    "HEB 220" => CrsHEB{width: 220.0_f64, height: 220.0_f64, area: 9100.0_f64, thickness_web: 9.5_f64, thickness_flange: 16.0_f64, radius: 18.0_f64, area_shear_y: 7040.000000000001_f64, w_elastic_y: 736000.0_f64, w_plastic_y: 828000.0_f64, inertia_y: 80900000.0_f64, area_shear_z: 1938.0_f64, w_elastic_z: 258000.0_f64, w_plastic_z: 394000.0_f64, inertia_z: 28400000.0_f64},
    "HEB 240" => CrsHEB{width: 240.0_f64, height: 240.0_f64, area: 10600.0_f64, thickness_web: 10.0_f64, thickness_flange: 17.0_f64, radius: 21.0_f64, area_shear_y: 8159.999999999999_f64, w_elastic_y: 938000.0_f64, w_plastic_y: 1050000.0_f64, inertia_y: 112600000.0_f64, area_shear_z: 2230.0_f64, w_elastic_z: 327000.0_f64, w_plastic_z: 499000.0_f64, inertia_z: 39200000.0_f64},
    "HEB 260" => CrsHEB{width: 260.0_f64, height: 260.0_f64, area: 11800.0_f64, thickness_web: 10.0_f64, thickness_flange: 17.5_f64, radius: 24.0_f64, area_shear_y: 9100.0_f64, w_elastic_y: 1150000.0_f64, w_plastic_y: 1280000.0_f64, inertia_y: 149200000.0_f64, area_shear_z: 2425.0_f64, w_elastic_z: 395000.0_f64, w_plastic_z: 603000.0_f64, inertia_z: 51300000.0_f64},
    "HEB 280" => CrsHEB{width: 280.0_f64, height: 280.0_f64, area: 13100.0_f64, thickness_web: 10.5_f64, thickness_flange: 18.0_f64, radius: 24.0_f64, area_shear_y: 10080.0_f64, w_elastic_y: 1380000.0_f64, w_plastic_y: 1530000.0_f64, inertia_y: 192700000.0_f64, area_shear_z: 2751.0_f64, w_elastic_z: 471000.0_f64, w_plastic_z: 718000.0_f64, inertia_z: 65900000.0_f64},
    "HEB 300" => CrsHEB{width: 300.0_f64, height: 300.0_f64, area: 14900.0_f64, thickness_web: 11.0_f64, thickness_flange: 19.0_f64, radius: 27.0_f64, area_shear_y: 11400.0_f64, w_elastic_y: 1680000.0_f64, w_plastic_y: 1870000.0_f64, inertia_y: 251700000.0_f64, area_shear_z: 3091.0_f64, w_elastic_z: 571000.0_f64, w_plastic_z: 871000.0_f64, inertia_z: 85600000.0_f64},
    "HEB 320" => CrsHEB{width: 300.0_f64, height: 320.0_f64, area: 16100.0_f64, thickness_web: 11.5_f64, thickness_flange: 20.5_f64, radius: 27.0_f64, area_shear_y: 12300.0_f64, w_elastic_y: 1930000.0_f64, w_plastic_y: 2140000.0_f64, inertia_y: 308200000.0_f64, area_shear_z: 3444.0_f64, w_elastic_z: 616000.0_f64, w_plastic_z: 940000.0_f64, inertia_z: 92400000.0_f64},
    "HEB 340" => CrsHEB{width: 300.0_f64, height: 340.0_f64, area: 17100.0_f64, thickness_web: 12.0_f64, thickness_flange: 21.5_f64, radius: 27.0_f64, area_shear_y: 12900.0_f64, w_elastic_y: 2160000.0_f64, w_plastic_y: 2400000.0_f64, inertia_y: 366600000.0_f64, area_shear_z: 3822.0_f64, w_elastic_z: 646000.0_f64, w_plastic_z: 986000.0_f64, inertia_z: 96900000.0_f64},
    "HEB 360" => CrsHEB{width: 300.0_f64, height: 360.0_f64, area: 18100.0_f64, thickness_web: 12.5_f64, thickness_flange: 22.5_f64, radius: 27.0_f64, area_shear_y: 13500.0_f64, w_elastic_y: 2400000.0_f64, w_plastic_y: 2680000.0_f64, inertia_y: 431900000.0_f64, area_shear_z: 4219.0_f64, w_elastic_z: 676000.0_f64, w_plastic_z: 1030000.0_f64, inertia_z: 101400000.0_f64},
    "HEB 400" => CrsHEB{width: 300.0_f64, height: 400.0_f64, area: 19800.0_f64, thickness_web: 13.5_f64, thickness_flange: 24.0_f64, radius: 27.0_f64, area_shear_y: 14400.0_f64, w_elastic_y: 2880000.0_f64, w_plastic_y: 3240000.0_f64, inertia_y: 576800000.0_f64, area_shear_z: 5076.0_f64, w_elastic_z: 721000.0_f64, w_plastic_z: 1100000.0_f64, inertia_z: 108200000.0_f64},
    "HEB 450" => CrsHEB{width: 300.0_f64, height: 450.0_f64, area: 21800.0_f64, thickness_web: 14.0_f64, thickness_flange: 26.0_f64, radius: 27.0_f64, area_shear_y: 15600.0_f64, w_elastic_y: 3550000.0_f64, w_plastic_y: 3980000.0_f64, inertia_y: 798900000.0_f64, area_shear_z: 5936.0_f64, w_elastic_z: 781000.0_f64, w_plastic_z: 1200000.0_f64, inertia_z: 117200000.0_f64},
    "HEB 500" => CrsHEB{width: 300.0_f64, height: 500.0_f64, area: 23900.0_f64, thickness_web: 14.5_f64, thickness_flange: 28.0_f64, radius: 27.0_f64, area_shear_y: 16800.0_f64, w_elastic_y: 4290000.0_f64, w_plastic_y: 4820000.0_f64, inertia_y: 1072000000.0_f64, area_shear_z: 6844.0_f64, w_elastic_z: 842000.0_f64, w_plastic_z: 1290000.0_f64, inertia_z: 126200000.0_f64},
    "HEB 550" => CrsHEB{width: 300.0_f64, height: 550.0_f64, area: 25400.0_f64, thickness_web: 15.0_f64, thickness_flange: 29.0_f64, radius: 27.0_f64, area_shear_y: 17400.0_f64, w_elastic_y: 4970000.0_f64, w_plastic_y: 5600000.0_f64, inertia_y: 1367000000.0_f64, area_shear_z: 7815.000000000001_f64, w_elastic_z: 872000.0_f64, w_plastic_z: 1340000.0_f64, inertia_z: 130800000.0_f64},
    "HEB 600" => CrsHEB{width: 300.0_f64, height: 600.0_f64, area: 27000.0_f64, thickness_web: 15.5_f64, thickness_flange: 30.0_f64, radius: 27.0_f64, area_shear_y: 18000.0_f64, w_elastic_y: 5700000.0_f64, w_plastic_y: 6420000.0_f64, inertia_y: 1710000000.0_f64, area_shear_z: 8835.0_f64, w_elastic_z: 902000.0_f64, w_plastic_z: 1390000.0_f64, inertia_z: 135300000.0_f64},
    "HEB 650" => CrsHEB{width: 300.0_f64, height: 650.0_f64, area: 28600.0_f64, thickness_web: 16.0_f64, thickness_flange: 31.0_f64, radius: 27.0_f64, area_shear_y: 18600.0_f64, w_elastic_y: 6480000.0_f64, w_plastic_y: 7320000.0_f64, inertia_y: 2106000000.0_f64, area_shear_z: 9904.0_f64, w_elastic_z: 932000.0_f64, w_plastic_z: 1440000.0_f64, inertia_z: 139800000.0_f64},
    "HEB 700" => CrsHEB{width: 300.0_f64, height: 700.0_f64, area: 30600.0_f64, thickness_web: 17.0_f64, thickness_flange: 32.0_f64, radius: 27.0_f64, area_shear_y: 19200.0_f64, w_elastic_y: 7340000.0_f64, w_plastic_y: 8320000.0_f64, inertia_y: 2569000000.0_f64, area_shear_z: 11356.0_f64, w_elastic_z: 963000.0_f64, w_plastic_z: 1490000.0_f64, inertia_z: 144400000.0_f64},
    "HEB 800" => CrsHEB{width: 300.0_f64, height: 800.0_f64, area: 33400.0_f64, thickness_web: 17.5_f64, thickness_flange: 33.0_f64, radius: 30.0_f64, area_shear_y: 19800.0_f64, w_elastic_y: 8980000.0_f64, w_plastic_y: 10220000.0_f64, inertia_y: 3591000000.0_f64, area_shear_z: 13422.0_f64, w_elastic_z: 994000.0_f64, w_plastic_z: 1550000.0_f64, inertia_z: 149000000.0_f64},
    "HEB 900" => CrsHEB{width: 300.0_f64, height: 900.0_f64, area: 37100.0_f64, thickness_web: 18.5_f64, thickness_flange: 35.0_f64, radius: 30.0_f64, area_shear_y: 21000.0_f64, w_elastic_y: 10980000.0_f64, w_plastic_y: 12580000.0_f64, inertia_y: 4941000000.0_f64, area_shear_z: 16002.000000000002_f64, w_elastic_z: 1050000.0_f64, w_plastic_z: 1660000.0_f64, inertia_z: 158200000.0_f64},
    "HEB 1000" => CrsHEB{width: 300.0_f64, height: 1000.0_f64, area: 40000.0_f64, thickness_web: 19.0_f64, thickness_flange: 36.0_f64, radius: 30.0_f64, area_shear_y: 21600.0_f64, w_elastic_y: 12890000.0_f64, w_plastic_y: 14860000.0_f64, inertia_y: 6447000000.0_f64, area_shear_z: 18316.0_f64, w_elastic_z: 1090000.0_f64, w_plastic_z: 1710000.0_f64, inertia_z: 162800000.0_f64},

};
