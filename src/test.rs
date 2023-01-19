#[cfg(test)]
mod integrationtest {
    use crate::{
        crs::CrossSectionClassCase,
        crs::{heb::CrsHEB, CrossSectionClass, CrsLib, PRESETS},
        mat::steel::{Steel, Variant},
        mmb::columnbeam::ColumnBeam,
    };
    #[test]
    fn dbg_all_heb_crossection_classes_in_web_bending() {
        let sections = CrsLib::sections(&PRESETS::HEB);
        for section in sections {
            let crs = CrsHEB::from_key(section).unwrap();
            let mat = Steel::from(&Variant::S355);
            let cmb = ColumnBeam::new(Box::new(crs), mat);
            println!(
                "Section: {}, Cross_section_class:{}",
                section,
                cmb.cross_section_class(CrossSectionClassCase::WebBending)
            );
        }
    }
    #[test]
    fn dbg_all_heb_crossection_classes_in_web_compression() {
        let sections = CrsLib::sections(&PRESETS::HEB);
        for section in sections {
            let crs = CrsHEB::from_key(section).unwrap();
            let mat = Steel::from(&Variant::S355);
            let cmb = ColumnBeam::new(Box::new(crs), mat);
            println!(
                "Section: {}, Cross_section_class:{}",
                section,
                cmb.cross_section_class(CrossSectionClassCase::WebCompression)
            );
        }
    }

    #[test]
    fn heb1000_s355_web_gives_cross_section_class_1_in_pure_bending() {
        let crs = CrsHEB::from_key("HEB 1000").unwrap();
        let mat = Steel::from(&Variant::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebBending),
            CrossSectionClass::One
        );
    }
    #[test]
    fn heb1000_s355_web_gives_cross_section_class_4_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 1000").unwrap();
        let mat = Steel::from(&Variant::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Four
        );
    }
    #[test]
    fn heb800_s355_web_gives_cross_section_class_3_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 800").unwrap();
        let mat = Steel::from(&Variant::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Four
        );
    }
    #[test]
    fn heb100_s355_web_gives_cross_section_class_3_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 600").unwrap();
        let mat = Steel::from(&Variant::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Three
        );
    }
}
