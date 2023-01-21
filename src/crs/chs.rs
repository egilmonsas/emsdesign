use super::{CrossSection, CrossSectionClass, CrossSectionClassCase};
use crate::Axis;
use phf::phf_ordered_map;

#[derive(Debug, Clone)]
pub struct CrsCHS {
    diameter: f64,
    wall_thickness: f64,
    area: f64,
    area_shear: f64,
    w_elastic: f64,
    w_plastic: f64,
    inertia: f64,
}

impl CrsCHS {
    #[must_use]
    pub fn from_key(key: &str) -> Option<Self> {
        CHSLIB.get(key).cloned()
    }
}

impl Default for CrsCHS {
    fn default() -> Self {
        Self::from_key("Celsius 355 CHS 21.3x2.6")
            .expect("Could not extract section 'Celsius 355 CHS 21.3x2.6'")
    }
}
impl CrossSection for CrsCHS {
    fn variant(&self) -> super::Variant {
        super::Variant::CHS
    }
    fn width(&self) -> f64 {
        self.diameter
    }
    fn height(&self) -> f64 {
        self.diameter
    }
    fn area(&self) -> f64 {
        self.area
    }

    fn I(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => self.inertia,
        }
    }
    fn w_el(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => self.w_elastic,
        }
    }
    fn w_pl(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => self.w_plastic,
        }
    }

    fn area_shear(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => {
                todo!()
            }
            Axis::Y | Axis::Z => self.area_shear,
        }
    }
    fn cross_section_class(&self, epsilon: f64, _case: CrossSectionClassCase) -> CrossSectionClass {
        match self.diameter / self.wall_thickness {
            res if res <= 50.0 * epsilon.powi(2) => CrossSectionClass::One,
            res if res <= 70.0 * epsilon.powi(2) => CrossSectionClass::Two,
            res if res <= 90.0 * epsilon.powi(2) => CrossSectionClass::Three,
            _ => CrossSectionClass::Four,
        }
    }
}

#[allow(clippy::unreadable_literal)]
pub static CHSLIB: phf::OrderedMap<&'static str, CrsCHS> = phf_ordered_map! {
"Celsius 355 CHS 21.3x2.6" => CrsCHS{diameter: 21.3_f64, wall_thickness: 2.6_f64, area: 153.0_f64, area_shear: 153.0_f64, w_elastic: 640.0_f64, w_plastic: 920.0_f64, inertia: 6800.000000000001_f64},
"Celsius 355 CHS 21.3x2.9" => CrsCHS{diameter: 21.3_f64, wall_thickness: 2.9_f64, area: 168.0_f64, area_shear: 168.0_f64, w_elastic: 680.0_f64, w_plastic: 990.0_f64, inertia: 7300.0_f64},
"Celsius 355 CHS 21.3x3.2" => CrsCHS{diameter: 21.3_f64, wall_thickness: 3.2_f64, area: 182.0_f64, area_shear: 182.0_f64, w_elastic: 720.0_f64, w_plastic: 1060.0_f64, inertia: 7700.0_f64},
"Celsius 355 CHS 26.9x2.6" => CrsCHS{diameter: 26.9_f64, wall_thickness: 2.6_f64, area: 198.0_f64, area_shear: 198.0_f64, w_elastic: 1100.0_f64, w_plastic: 1540.0_f64, inertia: 14800.0_f64},
"Celsius 355 CHS 26.9x2.9" => CrsCHS{diameter: 26.9_f64, wall_thickness: 2.9_f64, area: 219.0_f64, area_shear: 219.0_f64, w_elastic: 1190.0_f64, w_plastic: 1680.0_f64, inertia: 16000.0_f64},
"Celsius 355 CHS 26.9x3.2" => CrsCHS{diameter: 26.9_f64, wall_thickness: 3.2_f64, area: 238.0_f64, area_shear: 238.0_f64, w_elastic: 1270.0_f64, w_plastic: 1810.0_f64, inertia: 17000.0_f64},
"Celsius 355 CHS 26.9x3.6" => CrsCHS{diameter: 26.9_f64, wall_thickness: 3.6_f64, area: 264.0_f64, area_shear: 264.0_f64, w_elastic: 1360.0_f64, w_plastic: 1970.0_f64, inertia: 18300.0_f64},
"Celsius 355 CHS 33.7x2.6" => CrsCHS{diameter: 33.7_f64, wall_thickness: 2.6_f64, area: 254.0_f64, area_shear: 254.0_f64, w_elastic: 1840.0_f64, w_plastic: 2520.0_f64, inertia: 30900.0_f64},
"Celsius 355 CHS 33.7x2.9" => CrsCHS{diameter: 33.7_f64, wall_thickness: 2.9_f64, area: 281.0_f64, area_shear: 281.0_f64, w_elastic: 1990.0_f64, w_plastic: 2760.0_f64, inertia: 33600.0_f64},
"Celsius 355 CHS 33.7x3.2" => CrsCHS{diameter: 33.7_f64, wall_thickness: 3.2_f64, area: 307.0_f64, area_shear: 307.0_f64, w_elastic: 2140.0_f64, w_plastic: 2990.0_f64, inertia: 36000.0_f64},
"Celsius 355 CHS 33.7x3.6" => CrsCHS{diameter: 33.7_f64, wall_thickness: 3.6_f64, area: 340.0_f64, area_shear: 340.0_f64, w_elastic: 2320.0_f64, w_plastic: 3280.0_f64, inertia: 39100.0_f64},
"Celsius 355 CHS 33.7x4" => CrsCHS{diameter: 33.7_f64, wall_thickness: 4.0_f64, area: 373.0_f64, area_shear: 373.0_f64, w_elastic: 2490.0_f64, w_plastic: 3550.0_f64, inertia: 41900.00000000001_f64},
"Celsius 355 CHS 33.7x4.5" => CrsCHS{diameter: 33.7_f64, wall_thickness: 4.5_f64, area: 413.0_f64, area_shear: 413.0_f64, w_elastic: 2670.0_f64, w_plastic: 3870.0_f64, inertia: 45000.0_f64},
"Celsius 355 CHS 42.4x2.6" => CrsCHS{diameter: 42.4_f64, wall_thickness: 2.6_f64, area: 325.0_f64, area_shear: 325.0_f64, w_elastic: 3050.0_f64, w_plastic: 4120.0_f64, inertia: 64600.0_f64},
"Celsius 355 CHS 42.4x2.9" => CrsCHS{diameter: 42.4_f64, wall_thickness: 2.9_f64, area: 360.0_f64, area_shear: 360.0_f64, w_elastic: 3330.0_f64, w_plastic: 4530.0_f64, inertia: 70600.0_f64},
"Celsius 355 CHS 42.4x3.2" => CrsCHS{diameter: 42.4_f64, wall_thickness: 3.2_f64, area: 394.0_f64, area_shear: 394.0_f64, w_elastic: 3590.0_f64, w_plastic: 4930.0_f64, inertia: 76200.0_f64},
"Celsius 355 CHS 42.4x3.6" => CrsCHS{diameter: 42.4_f64, wall_thickness: 3.6_f64, area: 438.99999999999994_f64, area_shear: 438.99999999999994_f64, w_elastic: 3930.0_f64, w_plastic: 5440.0_f64, inertia: 83300.0_f64},
"Celsius 355 CHS 42.4x4" => CrsCHS{diameter: 42.4_f64, wall_thickness: 4.0_f64, area: 483.0_f64, area_shear: 483.0_f64, w_elastic: 4240.0_f64, w_plastic: 5920.0_f64, inertia: 89900.0_f64},
"Celsius 355 CHS 42.4x4.5" => CrsCHS{diameter: 42.4_f64, wall_thickness: 4.5_f64, area: 536.0_f64, area_shear: 536.0_f64, w_elastic: 4600.0_f64, w_plastic: 6490.0_f64, inertia: 97600.0_f64},
"Celsius 355 CHS 48.3x2.6" => CrsCHS{diameter: 48.3_f64, wall_thickness: 2.6_f64, area: 373.0_f64, area_shear: 373.0_f64, w_elastic: 4050.0_f64, w_plastic: 5440.0_f64, inertia: 97800.0_f64},
"Celsius 355 CHS 48.3x2.9" => CrsCHS{diameter: 48.3_f64, wall_thickness: 2.9_f64, area: 413.99999999999994_f64, area_shear: 413.99999999999994_f64, w_elastic: 4430.0_f64, w_plastic: 5990.0_f64, inertia: 107000.0_f64},
"Celsius 355 CHS 48.3x3.2" => CrsCHS{diameter: 48.3_f64, wall_thickness: 3.2_f64, area: 453.0_f64, area_shear: 453.0_f64, w_elastic: 4800.0_f64, w_plastic: 6520.0_f64, inertia: 116000.0_f64},
"Celsius 355 CHS 48.3x3.6" => CrsCHS{diameter: 48.3_f64, wall_thickness: 3.6_f64, area: 505.99999999999994_f64, area_shear: 505.99999999999994_f64, w_elastic: 5260.0_f64, w_plastic: 7210.0_f64, inertia: 127000.0_f64},
"Celsius 355 CHS 48.3x4" => CrsCHS{diameter: 48.3_f64, wall_thickness: 4.0_f64, area: 557.0_f64, area_shear: 557.0_f64, w_elastic: 5700.0_f64, w_plastic: 7870.0_f64, inertia: 138000.0_f64},
"Celsius 355 CHS 48.3x4.5" => CrsCHS{diameter: 48.3_f64, wall_thickness: 4.5_f64, area: 619.0_f64, area_shear: 619.0_f64, w_elastic: 6210.0_f64, w_plastic: 8660.0_f64, inertia: 150000.0_f64},
"Celsius 355 CHS 48.3x5" => CrsCHS{diameter: 48.3_f64, wall_thickness: 5.0_f64, area: 680.0_f64, area_shear: 680.0_f64, w_elastic: 6690.0_f64, w_plastic: 9420.0_f64, inertia: 162000.0_f64},
"Celsius 355 CHS 48.3x5.6" => CrsCHS{diameter: 48.3_f64, wall_thickness: 5.6_f64, area: 751.0_f64, area_shear: 751.0_f64, w_elastic: 7210.0_f64, w_plastic: 10300.0_f64, inertia: 174000.0_f64},
"Celsius 355 CHS 48.3x6.3" => CrsCHS{diameter: 48.3_f64, wall_thickness: 6.3_f64, area: 831.0_f64, area_shear: 831.0_f64, w_elastic: 7760.0_f64, w_plastic: 11200.0_f64, inertia: 187000.0_f64},
"Celsius 355 CHS 60.3x2.6" => CrsCHS{diameter: 60.3_f64, wall_thickness: 2.6_f64, area: 471.0_f64, area_shear: 471.0_f64, w_elastic: 6520.0_f64, w_plastic: 8660.0_f64, inertia: 197000.0_f64},
"Celsius 355 CHS 60.3x2.9" => CrsCHS{diameter: 60.3_f64, wall_thickness: 2.9_f64, area: 523.0_f64, area_shear: 523.0_f64, w_elastic: 7160.0_f64, w_plastic: 9560.0_f64, inertia: 216000.0_f64},
"Celsius 355 CHS 60.3x3.2" => CrsCHS{diameter: 60.3_f64, wall_thickness: 3.2_f64, area: 574.0_f64, area_shear: 574.0_f64, w_elastic: 7780.0_f64, w_plastic: 10400.0_f64, inertia: 235000.0_f64},
"Celsius 355 CHS 60.3x3.6" => CrsCHS{diameter: 60.3_f64, wall_thickness: 3.6_f64, area: 641.0_f64, area_shear: 641.0_f64, w_elastic: 8580.0_f64, w_plastic: 11600.0_f64, inertia: 259000.0_f64},
"Celsius 355 CHS 60.3x4" => CrsCHS{diameter: 60.3_f64, wall_thickness: 4.0_f64, area: 707.0_f64, area_shear: 707.0_f64, w_elastic: 9340.0_f64, w_plastic: 12700.0_f64, inertia: 282000.0_f64},
"Celsius 355 CHS 60.3x4.5" => CrsCHS{diameter: 60.3_f64, wall_thickness: 4.5_f64, area: 789.0_f64, area_shear: 789.0_f64, w_elastic: 10200.0_f64, w_plastic: 14000.0_f64, inertia: 309000.0_f64},
"Celsius 355 CHS 60.3x5" => CrsCHS{diameter: 60.3_f64, wall_thickness: 5.0_f64, area: 869.0_f64, area_shear: 869.0_f64, w_elastic: 11100.0_f64, w_plastic: 15300.0_f64, inertia: 335000.0_f64},
"Celsius 355 CHS 60.3x5.6" => CrsCHS{diameter: 60.3_f64, wall_thickness: 5.6_f64, area: 961.9999999999999_f64, area_shear: 961.9999999999999_f64, w_elastic: 12100.0_f64, w_plastic: 16800.0_f64, inertia: 364000.0_f64},
"Celsius 355 CHS 60.3x6.3" => CrsCHS{diameter: 60.3_f64, wall_thickness: 6.3_f64, area: 1070.0_f64, area_shear: 1070.0_f64, w_elastic: 13100.0_f64, w_plastic: 18500.0_f64, inertia: 395000.0_f64},
"Celsius 355 CHS 60.3x7.1" => CrsCHS{diameter: 60.3_f64, wall_thickness: 7.1_f64, area: 1190.0_f64, area_shear: 1190.0_f64, w_elastic: 14200.0_f64, w_plastic: 20200.0_f64, inertia: 427000.0_f64},
"Celsius 355 CHS 60.3x8" => CrsCHS{diameter: 60.3_f64, wall_thickness: 8.0_f64, area: 1310.0_f64, area_shear: 1310.0_f64, w_elastic: 15300.0_f64, w_plastic: 22100.0_f64, inertia: 460000.0_f64},
"Celsius 355 CHS 76.1x2.9" => CrsCHS{diameter: 76.1_f64, wall_thickness: 2.9_f64, area: 667.0_f64, area_shear: 667.0_f64, w_elastic: 11800.0_f64, w_plastic: 15500.0_f64, inertia: 447000.0_f64},
"Celsius 355 CHS 76.1x3.2" => CrsCHS{diameter: 76.1_f64, wall_thickness: 3.2_f64, area: 733.0_f64, area_shear: 733.0_f64, w_elastic: 12800.0_f64, w_plastic: 17000.0_f64, inertia: 488000.0_f64},
"Celsius 355 CHS 76.1x3.6" => CrsCHS{diameter: 76.1_f64, wall_thickness: 3.6_f64, area: 819.9999999999999_f64, area_shear: 819.9999999999999_f64, w_elastic: 14200.0_f64, w_plastic: 18900.0_f64, inertia: 540000.0_f64},
"Celsius 355 CHS 76.1x4" => CrsCHS{diameter: 76.1_f64, wall_thickness: 4.0_f64, area: 906.0_f64, area_shear: 906.0_f64, w_elastic: 15500.0_f64, w_plastic: 20800.0_f64, inertia: 591000.0_f64},
"Celsius 355 CHS 76.1x4.5" => CrsCHS{diameter: 76.1_f64, wall_thickness: 4.5_f64, area: 1010.0_f64, area_shear: 1010.0_f64, w_elastic: 17100.0_f64, w_plastic: 23100.0_f64, inertia: 651000.0_f64},
"Celsius 355 CHS 76.1x5" => CrsCHS{diameter: 76.1_f64, wall_thickness: 5.0_f64, area: 1120.0_f64, area_shear: 1120.0_f64, w_elastic: 18600.0_f64, w_plastic: 25300.0_f64, inertia: 709000.0_f64},
"Celsius 355 CHS 76.1x5.6" => CrsCHS{diameter: 76.1_f64, wall_thickness: 5.6_f64, area: 1240.0_f64, area_shear: 1240.0_f64, w_elastic: 20400.0_f64, w_plastic: 27900.0_f64, inertia: 775000.0_f64},
"Celsius 355 CHS 76.1x6.3" => CrsCHS{diameter: 76.1_f64, wall_thickness: 6.3_f64, area: 1380.0_f64, area_shear: 1380.0_f64, w_elastic: 22300.0_f64, w_plastic: 30800.0_f64, inertia: 848000.0_f64},
"Celsius 355 CHS 76.1x7.1" => CrsCHS{diameter: 76.1_f64, wall_thickness: 7.1_f64, area: 1540.0_f64, area_shear: 1540.0_f64, w_elastic: 24300.0_f64, w_plastic: 33900.0_f64, inertia: 926000.0_f64},
"Celsius 355 CHS 76.1x8" => CrsCHS{diameter: 76.1_f64, wall_thickness: 8.0_f64, area: 1710.0000000000002_f64, area_shear: 1710.0000000000002_f64, w_elastic: 26400.0_f64, w_plastic: 37300.0_f64, inertia: 1010000.0_f64},
"Celsius 355 CHS 88.9x2.9" => CrsCHS{diameter: 88.9_f64, wall_thickness: 2.9_f64, area: 784.0_f64, area_shear: 784.0_f64, w_elastic: 16300.0_f64, w_plastic: 21500.0_f64, inertia: 725000.0_f64},
"Celsius 355 CHS 88.9x3.2" => CrsCHS{diameter: 88.9_f64, wall_thickness: 3.2_f64, area: 861.9999999999999_f64, area_shear: 861.9999999999999_f64, w_elastic: 17800.0_f64, w_plastic: 23500.0_f64, inertia: 792000.0_f64},
"Celsius 355 CHS 88.9x3.6" => CrsCHS{diameter: 88.9_f64, wall_thickness: 3.6_f64, area: 965.0_f64, area_shear: 965.0_f64, w_elastic: 19800.0_f64, w_plastic: 26200.0_f64, inertia: 879000.0_f64},
"Celsius 355 CHS 88.9x4" => CrsCHS{diameter: 88.9_f64, wall_thickness: 4.0_f64, area: 1070.0_f64, area_shear: 1070.0_f64, w_elastic: 21700.0_f64, w_plastic: 28900.0_f64, inertia: 963000.0_f64},
"Celsius 355 CHS 88.9x4.5" => CrsCHS{diameter: 88.9_f64, wall_thickness: 4.5_f64, area: 1190.0_f64, area_shear: 1190.0_f64, w_elastic: 24000.0_f64, w_plastic: 32100.0_f64, inertia: 1070000.0_f64},
"Celsius 355 CHS 88.9x5" => CrsCHS{diameter: 88.9_f64, wall_thickness: 5.0_f64, area: 1320.0_f64, area_shear: 1320.0_f64, w_elastic: 26200.0_f64, w_plastic: 35200.0_f64, inertia: 1160000.0_f64},
"Celsius 355 CHS 88.9x5.6" => CrsCHS{diameter: 88.9_f64, wall_thickness: 5.6_f64, area: 1470.0_f64, area_shear: 1470.0_f64, w_elastic: 28700.0_f64, w_plastic: 38900.0_f64, inertia: 1280000.0_f64},
"Celsius 355 CHS 88.9x6.3" => CrsCHS{diameter: 88.9_f64, wall_thickness: 6.3_f64, area: 1630.0_f64, area_shear: 1630.0_f64, w_elastic: 31500.0_f64, w_plastic: 43100.0_f64, inertia: 1400000.0_f64},
"Celsius 355 CHS 88.9x7.1" => CrsCHS{diameter: 88.9_f64, wall_thickness: 7.1_f64, area: 1820.0_f64, area_shear: 1820.0_f64, w_elastic: 34600.0_f64, w_plastic: 47600.0_f64, inertia: 1540000.0_f64},
"Celsius 355 CHS 88.9x8" => CrsCHS{diameter: 88.9_f64, wall_thickness: 8.0_f64, area: 2030.0_f64, area_shear: 2030.0_f64, w_elastic: 37800.0_f64, w_plastic: 52500.0_f64, inertia: 1680000.0_f64},
"Celsius 355 CHS 101.6x3.6" => CrsCHS{diameter: 101.6_f64, wall_thickness: 3.6_f64, area: 1110.0_f64, area_shear: 1110.0_f64, w_elastic: 26200.0_f64, w_plastic: 34600.0_f64, inertia: 1330000.0_f64},
"Celsius 355 CHS 101.6x4" => CrsCHS{diameter: 101.6_f64, wall_thickness: 4.0_f64, area: 1230.0_f64, area_shear: 1230.0_f64, w_elastic: 28800.0_f64, w_plastic: 38100.0_f64, inertia: 1460000.0_f64},
"Celsius 355 CHS 101.6x4.5" => CrsCHS{diameter: 101.6_f64, wall_thickness: 4.5_f64, area: 1370.0_f64, area_shear: 1370.0_f64, w_elastic: 31900.0_f64, w_plastic: 42500.0_f64, inertia: 1620000.0_f64},
"Celsius 355 CHS 101.6x5" => CrsCHS{diameter: 101.6_f64, wall_thickness: 5.0_f64, area: 1520.0_f64, area_shear: 1520.0_f64, w_elastic: 34900.0_f64, w_plastic: 46700.0_f64, inertia: 1770000.0_f64},
"Celsius 355 CHS 101.6x5.6" => CrsCHS{diameter: 101.6_f64, wall_thickness: 5.6_f64, area: 1689.9999999999998_f64, area_shear: 1689.9999999999998_f64, w_elastic: 38400.0_f64, w_plastic: 51700.0_f64, inertia: 1950000.0_f64},
"Celsius 355 CHS 101.6x6.3" => CrsCHS{diameter: 101.6_f64, wall_thickness: 6.3_f64, area: 1889.9999999999998_f64, area_shear: 1889.9999999999998_f64, w_elastic: 42300.0_f64, w_plastic: 57300.0_f64, inertia: 2150000.0_f64},
"Celsius 355 CHS 101.6x7.1" => CrsCHS{diameter: 101.6_f64, wall_thickness: 7.1_f64, area: 2110.0_f64, area_shear: 2110.0_f64, w_elastic: 46600.0_f64, w_plastic: 63500.0_f64, inertia: 2370000.0_f64},
"Celsius 355 CHS 101.6x8" => CrsCHS{diameter: 101.6_f64, wall_thickness: 8.0_f64, area: 2350.0_f64, area_shear: 2350.0_f64, w_elastic: 51100.0_f64, w_plastic: 70300.0_f64, inertia: 2600000.0_f64},
"Celsius 355 CHS 114.3x3.6" => CrsCHS{diameter: 114.3_f64, wall_thickness: 3.6_f64, area: 1250.0_f64, area_shear: 1250.0_f64, w_elastic: 33600.0_f64, w_plastic: 44100.0_f64, inertia: 1920000.0_f64},
"Celsius 355 CHS 114.3x4" => CrsCHS{diameter: 114.3_f64, wall_thickness: 4.0_f64, area: 1390.0_f64, area_shear: 1390.0_f64, w_elastic: 36900.0_f64, w_plastic: 48700.0_f64, inertia: 2110000.0_f64},
"Celsius 355 CHS 114.3x4.5" => CrsCHS{diameter: 114.3_f64, wall_thickness: 4.5_f64, area: 1550.0_f64, area_shear: 1550.0_f64, w_elastic: 41000.0_f64, w_plastic: 54300.0_f64, inertia: 2340000.0_f64},
"Celsius 355 CHS 114.3x5" => CrsCHS{diameter: 114.3_f64, wall_thickness: 5.0_f64, area: 1720.0_f64, area_shear: 1720.0_f64, w_elastic: 45000.0_f64, w_plastic: 59800.0_f64, inertia: 2570000.0_f64},
"Celsius 355 CHS 114.3x5.6" => CrsCHS{diameter: 114.3_f64, wall_thickness: 5.6_f64, area: 1910.0000000000002_f64, area_shear: 1910.0000000000002_f64, w_elastic: 49600.0_f64, w_plastic: 66200.0_f64, inertia: 2830000.0_f64},
"Celsius 355 CHS 114.3x6.3" => CrsCHS{diameter: 114.3_f64, wall_thickness: 6.3_f64, area: 2140.0_f64, area_shear: 2140.0_f64, w_elastic: 54700.0_f64, w_plastic: 73600.0_f64, inertia: 3130000.0_f64},
"Celsius 355 CHS 114.3x7.1" => CrsCHS{diameter: 114.3_f64, wall_thickness: 7.1_f64, area: 2390.0_f64, area_shear: 2390.0_f64, w_elastic: 60400.0_f64, w_plastic: 81700.0_f64, inertia: 3450000.0_f64},
"Celsius 355 CHS 114.3x8" => CrsCHS{diameter: 114.3_f64, wall_thickness: 8.0_f64, area: 2670.0_f64, area_shear: 2670.0_f64, w_elastic: 66400.0_f64, w_plastic: 90600.0_f64, inertia: 3790000.0_f64},
"Celsius 355 CHS 139.7x3.6" => CrsCHS{diameter: 139.7_f64, wall_thickness: 3.6_f64, area: 1540.0_f64, area_shear: 1540.0_f64, w_elastic: 51100.0_f64, w_plastic: 66700.0_f64, inertia: 3570000.0_f64},
"Celsius 355 CHS 139.7x4" => CrsCHS{diameter: 139.7_f64, wall_thickness: 4.0_f64, area: 1710.0000000000002_f64, area_shear: 1710.0000000000002_f64, w_elastic: 56200.0_f64, w_plastic: 73700.0_f64, inertia: 3930000.0_f64},
"Celsius 355 CHS 139.7x4.5" => CrsCHS{diameter: 139.7_f64, wall_thickness: 4.5_f64, area: 1910.0000000000002_f64, area_shear: 1910.0000000000002_f64, w_elastic: 62600.0_f64, w_plastic: 82300.0_f64, inertia: 4370000.0_f64},
"Celsius 355 CHS 139.7x5" => CrsCHS{diameter: 139.7_f64, wall_thickness: 5.0_f64, area: 2120.0_f64, area_shear: 2120.0_f64, w_elastic: 68800.0_f64, w_plastic: 90800.0_f64, inertia: 4810000.0_f64},
"Celsius 355 CHS 139.7x5.6" => CrsCHS{diameter: 139.7_f64, wall_thickness: 5.6_f64, area: 2360.0_f64, area_shear: 2360.0_f64, w_elastic: 76100.0_f64, w_plastic: 101000.0_f64, inertia: 5310000.0_f64},
"Celsius 355 CHS 139.7x6.3" => CrsCHS{diameter: 139.7_f64, wall_thickness: 6.3_f64, area: 2640.0_f64, area_shear: 2640.0_f64, w_elastic: 84300.0_f64, w_plastic: 112000.0_f64, inertia: 5890000.0_f64},
"Celsius 355 CHS 139.7x7.1" => CrsCHS{diameter: 139.7_f64, wall_thickness: 7.1_f64, area: 2960.0_f64, area_shear: 2960.0_f64, w_elastic: 93300.0_f64, w_plastic: 125000.0_f64, inertia: 6520000.0_f64},
"Celsius 355 CHS 139.7x8" => CrsCHS{diameter: 139.7_f64, wall_thickness: 8.0_f64, area: 3310.0_f64, area_shear: 3310.0_f64, w_elastic: 103000.0_f64, w_plastic: 139000.0_f64, inertia: 7200000.0_f64},
"Celsius 355 CHS 139.7x10" => CrsCHS{diameter: 139.7_f64, wall_thickness: 10.0_f64, area: 4070.0000000000005_f64, area_shear: 4070.0000000000005_f64, w_elastic: 123000.0_f64, w_plastic: 169000.0_f64, inertia: 8620000.0_f64},
"Celsius 355 CHS 168.3x5" => CrsCHS{diameter: 168.3_f64, wall_thickness: 5.0_f64, area: 2570.0_f64, area_shear: 2570.0_f64, w_elastic: 102000.0_f64, w_plastic: 133000.0_f64, inertia: 8560000.0_f64},
"Celsius 355 CHS 168.3x5.6" => CrsCHS{diameter: 168.3_f64, wall_thickness: 5.6_f64, area: 2860.0_f64, area_shear: 2860.0_f64, w_elastic: 113000.0_f64, w_plastic: 148000.0_f64, inertia: 9480000.0_f64},
"Celsius 355 CHS 168.3x6.3" => CrsCHS{diameter: 168.3_f64, wall_thickness: 6.3_f64, area: 3210.0_f64, area_shear: 3210.0_f64, w_elastic: 125000.0_f64, w_plastic: 165000.0_f64, inertia: 10530000.0_f64},
"Celsius 355 CHS 168.3x7.1" => CrsCHS{diameter: 168.3_f64, wall_thickness: 7.1_f64, area: 3600.0_f64, area_shear: 3600.0_f64, w_elastic: 139000.0_f64, w_plastic: 185000.0_f64, inertia: 11700000.0_f64},
"Celsius 355 CHS 168.3x8" => CrsCHS{diameter: 168.3_f64, wall_thickness: 8.0_f64, area: 4029.9999999999995_f64, area_shear: 4029.9999999999995_f64, w_elastic: 154000.0_f64, w_plastic: 206000.0_f64, inertia: 12970000.0_f64},
"Celsius 355 CHS 168.3x10" => CrsCHS{diameter: 168.3_f64, wall_thickness: 10.0_f64, area: 4970.0_f64, area_shear: 4970.0_f64, w_elastic: 186000.0_f64, w_plastic: 251000.0_f64, inertia: 15640000.0_f64},
"Celsius 355 CHS 168.3x11" => CrsCHS{diameter: 168.3_f64, wall_thickness: 11.0_f64, area: 5440.0_f64, area_shear: 5440.0_f64, w_elastic: 201000.0_f64, w_plastic: 273000.0_f64, inertia: 16890000.0_f64},
"Celsius 355 CHS 168.3x12.5" => CrsCHS{diameter: 168.3_f64, wall_thickness: 12.5_f64, area: 6120.0_f64, area_shear: 6120.0_f64, w_elastic: 222000.0_f64, w_plastic: 304000.0_f64, inertia: 18680000.0_f64},
"Celsius 355 CHS 193.7x5" => CrsCHS{diameter: 193.7_f64, wall_thickness: 5.0_f64, area: 2960.0_f64, area_shear: 2960.0_f64, w_elastic: 136000.0_f64, w_plastic: 178000.0_f64, inertia: 13200000.0_f64},
"Celsius 355 CHS 193.7x5.6" => CrsCHS{diameter: 193.7_f64, wall_thickness: 5.6_f64, area: 3310.0_f64, area_shear: 3310.0_f64, w_elastic: 151000.0_f64, w_plastic: 198000.0_f64, inertia: 14650000.0_f64},
"Celsius 355 CHS 193.7x6.3" => CrsCHS{diameter: 193.7_f64, wall_thickness: 6.3_f64, area: 3710.0_f64, area_shear: 3710.0_f64, w_elastic: 168000.0_f64, w_plastic: 221000.0_f64, inertia: 16300000.0_f64},
"Celsius 355 CHS 193.7x7.1" => CrsCHS{diameter: 193.7_f64, wall_thickness: 7.1_f64, area: 4160.0_f64, area_shear: 4160.0_f64, w_elastic: 187000.0_f64, w_plastic: 247000.0_f64, inertia: 18140000.0_f64},
"Celsius 355 CHS 193.7x8" => CrsCHS{diameter: 193.7_f64, wall_thickness: 8.0_f64, area: 4670.0_f64, area_shear: 4670.0_f64, w_elastic: 208000.0_f64, w_plastic: 276000.0_f64, inertia: 20160000.0_f64},
"Celsius 355 CHS 193.7x10" => CrsCHS{diameter: 193.7_f64, wall_thickness: 10.0_f64, area: 5770.0_f64, area_shear: 5770.0_f64, w_elastic: 252000.0_f64, w_plastic: 338000.0_f64, inertia: 24420000.0_f64},
"Celsius 355 CHS 193.7x11" => CrsCHS{diameter: 193.7_f64, wall_thickness: 11.0_f64, area: 6310.0_f64, area_shear: 6310.0_f64, w_elastic: 273000.0_f64, w_plastic: 368000.0_f64, inertia: 26440000.0_f64},
"Celsius 355 CHS 193.7x12.5" => CrsCHS{diameter: 193.7_f64, wall_thickness: 12.5_f64, area: 7120.0_f64, area_shear: 7120.0_f64, w_elastic: 303000.0_f64, w_plastic: 411000.0_f64, inertia: 29340000.0_f64},
"Celsius 355 CHS 193.7x14.2" => CrsCHS{diameter: 193.7_f64, wall_thickness: 14.2_f64, area: 8009.999999999999_f64, area_shear: 8009.999999999999_f64, w_elastic: 335000.0_f64, w_plastic: 458000.0_f64, inertia: 32450000.0_f64},
"Celsius 355 CHS 193.7x16" => CrsCHS{diameter: 193.7_f64, wall_thickness: 16.0_f64, area: 8930.0_f64, area_shear: 8930.0_f64, w_elastic: 367000.0_f64, w_plastic: 507000.0_f64, inertia: 35540000.0_f64},
"Celsius 355 CHS 219.1x4.5" => CrsCHS{diameter: 219.1_f64, wall_thickness: 4.5_f64, area: 3030.0_f64, area_shear: 3030.0_f64, w_elastic: 159000.0_f64, w_plastic: 207000.0_f64, inertia: 17470000.0_f64},
"Celsius 355 CHS 219.1x5" => CrsCHS{diameter: 219.1_f64, wall_thickness: 5.0_f64, area: 3360.0_f64, area_shear: 3360.0_f64, w_elastic: 176000.0_f64, w_plastic: 229000.0_f64, inertia: 19280000.0_f64},
"Celsius 355 CHS 219.1x5.6" => CrsCHS{diameter: 219.1_f64, wall_thickness: 5.6_f64, area: 3760.0_f64, area_shear: 3760.0_f64, w_elastic: 195000.0_f64, w_plastic: 255000.0_f64, inertia: 21420000.0_f64},
"Celsius 355 CHS 219.1x6.3" => CrsCHS{diameter: 219.1_f64, wall_thickness: 6.3_f64, area: 4210.0_f64, area_shear: 4210.0_f64, w_elastic: 218000.0_f64, w_plastic: 285000.0_f64, inertia: 23860000.0_f64},
"Celsius 355 CHS 219.1x7.1" => CrsCHS{diameter: 219.1_f64, wall_thickness: 7.1_f64, area: 4730.0_f64, area_shear: 4730.0_f64, w_elastic: 243000.0_f64, w_plastic: 319000.0_f64, inertia: 26600000.0_f64},
"Celsius 355 CHS 219.1x8" => CrsCHS{diameter: 219.1_f64, wall_thickness: 8.0_f64, area: 5310.0_f64, area_shear: 5310.0_f64, w_elastic: 270000.0_f64, w_plastic: 357000.0_f64, inertia: 29600000.0_f64},
"Celsius 355 CHS 219.1x10" => CrsCHS{diameter: 219.1_f64, wall_thickness: 10.0_f64, area: 6570.0_f64, area_shear: 6570.0_f64, w_elastic: 328000.0_f64, w_plastic: 438000.0_f64, inertia: 35980000.0_f64},
"Celsius 355 CHS 219.1x11" => CrsCHS{diameter: 219.1_f64, wall_thickness: 11.0_f64, area: 7190.000000000001_f64, area_shear: 7190.000000000001_f64, w_elastic: 356000.0_f64, w_plastic: 477000.0_f64, inertia: 39040000.0_f64},
"Celsius 355 CHS 219.1x12.5" => CrsCHS{diameter: 219.1_f64, wall_thickness: 12.5_f64, area: 8109.999999999999_f64, area_shear: 8109.999999999999_f64, w_elastic: 397000.0_f64, w_plastic: 534000.0_f64, inertia: 43450000.0_f64},
"Celsius 355 CHS 219.1x14.2" => CrsCHS{diameter: 219.1_f64, wall_thickness: 14.2_f64, area: 9140.0_f64, area_shear: 9140.0_f64, w_elastic: 440000.0_f64, w_plastic: 597000.0_f64, inertia: 48200000.0_f64},
"Celsius 355 CHS 219.1x16" => CrsCHS{diameter: 219.1_f64, wall_thickness: 16.0_f64, area: 10200.0_f64, area_shear: 10200.0_f64, w_elastic: 483000.0_f64, w_plastic: 661000.0_f64, inertia: 52970000.0_f64},
"Celsius 355 CHS 244.5x5" => CrsCHS{diameter: 244.5_f64, wall_thickness: 5.0_f64, area: 3760.0_f64, area_shear: 3760.0_f64, w_elastic: 221000.0_f64, w_plastic: 287000.0_f64, inertia: 26990000.0_f64},
"Celsius 355 CHS 244.5x5.6" => CrsCHS{diameter: 244.5_f64, wall_thickness: 5.6_f64, area: 4200.0_f64, area_shear: 4200.0_f64, w_elastic: 245000.0_f64, w_plastic: 320000.0_f64, inertia: 30000000.0_f64},
"Celsius 355 CHS 244.5x6.3" => CrsCHS{diameter: 244.5_f64, wall_thickness: 6.3_f64, area: 4710.0_f64, area_shear: 4710.0_f64, w_elastic: 274000.0_f64, w_plastic: 358000.0_f64, inertia: 33460000.0_f64},
"Celsius 355 CHS 244.5x7.1" => CrsCHS{diameter: 244.5_f64, wall_thickness: 7.1_f64, area: 5300.0_f64, area_shear: 5300.0_f64, w_elastic: 305000.0_f64, w_plastic: 400000.0_f64, inertia: 37340000.0_f64},
"Celsius 355 CHS 244.5x8" => CrsCHS{diameter: 244.5_f64, wall_thickness: 8.0_f64, area: 5940.0_f64, area_shear: 5940.0_f64, w_elastic: 340000.0_f64, w_plastic: 448000.0_f64, inertia: 41600000.0_f64},
"Celsius 355 CHS 244.5x10" => CrsCHS{diameter: 244.5_f64, wall_thickness: 10.0_f64, area: 7370.0_f64, area_shear: 7370.0_f64, w_elastic: 415000.0_f64, w_plastic: 550000.0_f64, inertia: 50730000.0_f64},
"Celsius 355 CHS 244.5x11" => CrsCHS{diameter: 244.5_f64, wall_thickness: 11.0_f64, area: 8070.0_f64, area_shear: 8070.0_f64, w_elastic: 451000.0_f64, w_plastic: 600000.0_f64, inertia: 55120000.0_f64},
"Celsius 355 CHS 244.5x12.5" => CrsCHS{diameter: 244.5_f64, wall_thickness: 12.5_f64, area: 9110.0_f64, area_shear: 9110.0_f64, w_elastic: 503000.0_f64, w_plastic: 673000.0_f64, inertia: 61470000.0_f64},
"Celsius 355 CHS 244.5x14.2" => CrsCHS{diameter: 244.5_f64, wall_thickness: 14.2_f64, area: 10300.0_f64, area_shear: 10300.0_f64, w_elastic: 559000.0_f64, w_plastic: 754000.0_f64, inertia: 68370000.0_f64},
"Celsius 355 CHS 244.5x16" => CrsCHS{diameter: 244.5_f64, wall_thickness: 16.0_f64, area: 11500.0_f64, area_shear: 11500.0_f64, w_elastic: 616000.0_f64, w_plastic: 837000.0_f64, inertia: 75330000.0_f64},
"Celsius 355 CHS 273x5" => CrsCHS{diameter: 273.0_f64, wall_thickness: 5.0_f64, area: 4210.0_f64, area_shear: 4210.0_f64, w_elastic: 277000.0_f64, w_plastic: 359000.0_f64, inertia: 37810000.0_f64},
"Celsius 355 CHS 273x5.6" => CrsCHS{diameter: 273.0_f64, wall_thickness: 5.6_f64, area: 4700.0_f64, area_shear: 4700.0_f64, w_elastic: 308000.0_f64, w_plastic: 400000.0_f64, inertia: 42070000.0_f64},
"Celsius 355 CHS 273x6.3" => CrsCHS{diameter: 273.0_f64, wall_thickness: 6.3_f64, area: 5280.0_f64, area_shear: 5280.0_f64, w_elastic: 344000.0_f64, w_plastic: 448000.0_f64, inertia: 46960000.0_f64},
"Celsius 355 CHS 273x7.1" => CrsCHS{diameter: 273.0_f64, wall_thickness: 7.1_f64, area: 5930.0_f64, area_shear: 5930.0_f64, w_elastic: 384000.0_f64, w_plastic: 502000.0_f64, inertia: 52450000.0_f64},
"Celsius 355 CHS 273x8" => CrsCHS{diameter: 273.0_f64, wall_thickness: 8.0_f64, area: 6659.999999999999_f64, area_shear: 6659.999999999999_f64, w_elastic: 429000.0_f64, w_plastic: 562000.0_f64, inertia: 58520000.0_f64},
"Celsius 355 CHS 273x10" => CrsCHS{diameter: 273.0_f64, wall_thickness: 10.0_f64, area: 8260.0_f64, area_shear: 8260.0_f64, w_elastic: 524000.0_f64, w_plastic: 692000.0_f64, inertia: 71540000.0_f64},
"Celsius 355 CHS 273x11" => CrsCHS{diameter: 273.0_f64, wall_thickness: 11.0_f64, area: 9050.0_f64, area_shear: 9050.0_f64, w_elastic: 570000.0_f64, w_plastic: 756000.0_f64, inertia: 77830000.0_f64},
"Celsius 355 CHS 273x12.5" => CrsCHS{diameter: 273.0_f64, wall_thickness: 12.5_f64, area: 10200.0_f64, area_shear: 10200.0_f64, w_elastic: 637000.0_f64, w_plastic: 849000.0_f64, inertia: 86970000.0_f64},
"Celsius 355 CHS 273x14.2" => CrsCHS{diameter: 273.0_f64, wall_thickness: 14.2_f64, area: 11500.0_f64, area_shear: 11500.0_f64, w_elastic: 710000.0_f64, w_plastic: 952000.0_f64, inertia: 96950000.0_f64},
"Celsius 355 CHS 273x16" => CrsCHS{diameter: 273.0_f64, wall_thickness: 16.0_f64, area: 12900.0_f64, area_shear: 12900.0_f64, w_elastic: 784000.0_f64, w_plastic: 1058000.0_f64, inertia: 107070000.0_f64},
"Celsius 355 CHS 323.9x5" => CrsCHS{diameter: 323.9_f64, wall_thickness: 5.0_f64, area: 5010.0_f64, area_shear: 5010.0_f64, w_elastic: 393000.0_f64, w_plastic: 509000.0_f64, inertia: 63690000.0_f64},
"Celsius 355 CHS 323.9x5.6" => CrsCHS{diameter: 323.9_f64, wall_thickness: 5.6_f64, area: 5600.0_f64, area_shear: 5600.0_f64, w_elastic: 438000.0_f64, w_plastic: 567000.0_f64, inertia: 70940000.0_f64},
"Celsius 355 CHS 323.9x6.3" => CrsCHS{diameter: 323.9_f64, wall_thickness: 6.3_f64, area: 6290.0_f64, area_shear: 6290.0_f64, w_elastic: 490000.0_f64, w_plastic: 636000.0_f64, inertia: 79290000.0_f64},
"Celsius 355 CHS 323.9x7.1" => CrsCHS{diameter: 323.9_f64, wall_thickness: 7.1_f64, area: 7070.0_f64, area_shear: 7070.0_f64, w_elastic: 548000.0_f64, w_plastic: 713000.0_f64, inertia: 88690000.0_f64},
"Celsius 355 CHS 323.9x8" => CrsCHS{diameter: 323.9_f64, wall_thickness: 8.0_f64, area: 7940.000000000001_f64, area_shear: 7940.000000000001_f64, w_elastic: 612000.0_f64, w_plastic: 799000.0_f64, inertia: 99100000.0_f64},
"Celsius 355 CHS 323.9x10" => CrsCHS{diameter: 323.9_f64, wall_thickness: 10.0_f64, area: 9860.0_f64, area_shear: 9860.0_f64, w_elastic: 751000.0_f64, w_plastic: 986000.0_f64, inertia: 121580000.0_f64},
"Celsius 355 CHS 323.9x11" => CrsCHS{diameter: 323.9_f64, wall_thickness: 11.0_f64, area: 10800.0_f64, area_shear: 10800.0_f64, w_elastic: 818000.0_f64, w_plastic: 1077000.0_f64, inertia: 132500000.0_f64},
"Celsius 355 CHS 323.9x12.5" => CrsCHS{diameter: 323.9_f64, wall_thickness: 12.5_f64, area: 12200.0_f64, area_shear: 12200.0_f64, w_elastic: 917000.0_f64, w_plastic: 1213000.0_f64, inertia: 148470000.0_f64},
"Celsius 355 CHS 323.9x14.2" => CrsCHS{diameter: 323.9_f64, wall_thickness: 14.2_f64, area: 13800.0_f64, area_shear: 13800.0_f64, w_elastic: 1025000.0_f64, w_plastic: 1363000.0_f64, inertia: 165990000.0_f64},
"Celsius 355 CHS 323.9x16" => CrsCHS{diameter: 323.9_f64, wall_thickness: 16.0_f64, area: 15500.0_f64, area_shear: 15500.0_f64, w_elastic: 1136000.0_f64, w_plastic: 1518000.0_f64, inertia: 183900000.0_f64},
"Celsius 355 CHS 355.6x6.3" => CrsCHS{diameter: 355.6_f64, wall_thickness: 6.3_f64, area: 6909.999999999999_f64, area_shear: 6909.999999999999_f64, w_elastic: 593000.0_f64, w_plastic: 769000.0_f64, inertia: 105470000.0_f64},
"Celsius 355 CHS 355.6x7.1" => CrsCHS{diameter: 355.6_f64, wall_thickness: 7.1_f64, area: 7770.0_f64, area_shear: 7770.0_f64, w_elastic: 664000.0_f64, w_plastic: 862000.0_f64, inertia: 118060000.0_f64},
"Celsius 355 CHS 355.6x8" => CrsCHS{diameter: 355.6_f64, wall_thickness: 8.0_f64, area: 8740.0_f64, area_shear: 8740.0_f64, w_elastic: 742000.0_f64, w_plastic: 967000.0_f64, inertia: 132010000.0_f64},
"Celsius 355 CHS 355.6x10" => CrsCHS{diameter: 355.6_f64, wall_thickness: 10.0_f64, area: 10900.0_f64, area_shear: 10900.0_f64, w_elastic: 912000.0_f64, w_plastic: 1195000.0_f64, inertia: 162230000.0_f64},
"Celsius 355 CHS 355.6x11" => CrsCHS{diameter: 355.6_f64, wall_thickness: 11.0_f64, area: 11900.0_f64, area_shear: 11900.0_f64, w_elastic: 995000.0_f64, w_plastic: 1307000.0_f64, inertia: 176950000.0_f64},
"Celsius 355 CHS 355.6x12.5" => CrsCHS{diameter: 355.6_f64, wall_thickness: 12.5_f64, area: 13500.0_f64, area_shear: 13500.0_f64, w_elastic: 1117000.0_f64, w_plastic: 1472000.0_f64, inertia: 198520000.0_f64},
"Celsius 355 CHS 355.6x14.2" => CrsCHS{diameter: 355.6_f64, wall_thickness: 14.2_f64, area: 15200.0_f64, area_shear: 15200.0_f64, w_elastic: 1250000.0_f64, w_plastic: 1656000.0_f64, inertia: 222270000.0_f64},
"Celsius 355 CHS 355.6x16" => CrsCHS{diameter: 355.6_f64, wall_thickness: 16.0_f64, area: 17100.0_f64, area_shear: 17100.0_f64, w_elastic: 1387000.0_f64, w_plastic: 1847000.0_f64, inertia: 246630000.0_f64},
"Celsius 355 CHS 406.4x10" => CrsCHS{diameter: 406.4_f64, wall_thickness: 10.0_f64, area: 12500.0_f64, area_shear: 12500.0_f64, w_elastic: 1205000.0_f64, w_plastic: 1572000.0_f64, inertia: 244760000.0_f64},
"Celsius 355 CHS 406.4x11" => CrsCHS{diameter: 406.4_f64, wall_thickness: 11.0_f64, area: 13700.0_f64, area_shear: 13700.0_f64, w_elastic: 1315000.0_f64, w_plastic: 1720000.0_f64, inertia: 267240000.0_f64},
"Celsius 355 CHS 406.4x12.5" => CrsCHS{diameter: 406.4_f64, wall_thickness: 12.5_f64, area: 15500.0_f64, area_shear: 15500.0_f64, w_elastic: 1478000.0_f64, w_plastic: 1940000.0_f64, inertia: 300310000.0_f64},
"Celsius 355 CHS 406.4x14.2" => CrsCHS{diameter: 406.4_f64, wall_thickness: 14.2_f64, area: 17500.0_f64, area_shear: 17500.0_f64, w_elastic: 1658000.0_f64, w_plastic: 2185000.0_f64, inertia: 336850000.0_f64},
"Celsius 355 CHS 406.4x16" => CrsCHS{diameter: 406.4_f64, wall_thickness: 16.0_f64, area: 19600.0_f64, area_shear: 19600.0_f64, w_elastic: 1843000.0_f64, w_plastic: 2440000.0_f64, inertia: 374490000.0_f64},
"Celsius 355 CHS 457x10" => CrsCHS{diameter: 457.0_f64, wall_thickness: 10.0_f64, area: 14000.0_f64, area_shear: 14000.0_f64, w_elastic: 1536000.0_f64, w_plastic: 1998000.0_f64, inertia: 350910000.0_f64},
"Celsius 355 CHS 457x11" => CrsCHS{diameter: 457.0_f64, wall_thickness: 11.0_f64, area: 15400.0_f64, area_shear: 15400.0_f64, w_elastic: 1678000.0_f64, w_plastic: 2189000.0_f64, inertia: 383460000.0_f64},
"Celsius 355 CHS 457x12.5" => CrsCHS{diameter: 457.0_f64, wall_thickness: 12.5_f64, area: 17500.0_f64, area_shear: 17500.0_f64, w_elastic: 1888000.0_f64, w_plastic: 2470000.0_f64, inertia: 431450000.0_f64},
"Celsius 355 CHS 457x14.2" => CrsCHS{diameter: 457.0_f64, wall_thickness: 14.2_f64, area: 19800.0_f64, area_shear: 19800.0_f64, w_elastic: 2121000.0_f64, w_plastic: 2785000.0_f64, inertia: 484640000.0_f64},
"Celsius 355 CHS 457x16" => CrsCHS{diameter: 457.0_f64, wall_thickness: 16.0_f64, area: 22200.0_f64, area_shear: 22200.0_f64, w_elastic: 2361000.0_f64, w_plastic: 3113000.0_f64, inertia: 539590000.0_f64},
"Celsius 355 CHS 508x10" => CrsCHS{diameter: 508.0_f64, wall_thickness: 10.0_f64, area: 15600.0_f64, area_shear: 15600.0_f64, w_elastic: 1910000.0_f64, w_plastic: 2480000.0_f64, inertia: 485200000.0_f64},
"Celsius 355 CHS 508x11" => CrsCHS{diameter: 508.0_f64, wall_thickness: 11.0_f64, area: 17200.0_f64, area_shear: 17200.0_f64, w_elastic: 2089000.0_f64, w_plastic: 2718000.0_f64, inertia: 530560000.0_f64},
"Celsius 355 CHS 508x12.5" => CrsCHS{diameter: 508.0_f64, wall_thickness: 12.5_f64, area: 19500.0_f64, area_shear: 19500.0_f64, w_elastic: 2353000.0_f64, w_plastic: 3070000.0_f64, inertia: 597550000.0_f64},
"Celsius 355 CHS 508x14.2" => CrsCHS{diameter: 508.0_f64, wall_thickness: 14.2_f64, area: 22000.0_f64, area_shear: 22000.0_f64, w_elastic: 2646000.0_f64, w_plastic: 3463000.0_f64, inertia: 671990000.0_f64},
"Celsius 355 CHS 508x16" => CrsCHS{diameter: 508.0_f64, wall_thickness: 16.0_f64, area: 24700.0_f64, area_shear: 24700.0_f64, w_elastic: 2949000.0_f64, w_plastic: 3874000.0_f64, inertia: 749090000.0_f64},
};
