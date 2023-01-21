#[cfg(test)]
mod integrationtest {
    use crate::{
        crs::CrossSectionClassCase,
        crs::{heb::CrsHEB, CrossSectionClass, CrossSectionLib, Variant},
        mat::steel::{Class, Steel},
        mmb::columnbeam::ColumnBeam,
    };
    #[test]
    fn dbg_all_heb_crossection_classes_in_web_bending() {
        let sections = CrossSectionLib::sections(&Variant::HEB);
        for section in sections {
            let crs = CrsHEB::from_key(section).expect("Could not extract section");
            let mat = Steel::from(&Class::S355);
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
        let sections = CrossSectionLib::sections(&Variant::HEB);
        for section in sections {
            let crs = CrsHEB::from_key(section).expect("Could not extract section");
            let mat = Steel::from(&Class::S355);
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
        let crs = CrsHEB::from_key("HEB 1000").expect("Could not extract section 'HEB 1000'");
        let mat = Steel::from(&Class::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebBending),
            CrossSectionClass::One
        );
    }
    #[test]
    fn heb1000_s355_web_gives_cross_section_class_4_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 1000").expect("Could not extract section 'HEB 1000'");
        let mat = Steel::from(&Class::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Four
        );
    }
    #[test]
    fn heb800_s355_web_gives_cross_section_class_3_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 800").expect("Could not extract section 'HEB 800'");
        let mat = Steel::from(&Class::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Four
        );
    }
    #[test]
    fn heb100_s355_web_gives_cross_section_class_3_in_pure_compression() {
        let crs = CrsHEB::from_key("HEB 600").expect("Could not extract section 'HEB 600'");
        let mat = Steel::from(&Class::S355);
        let cmb = ColumnBeam::new(Box::new(crs), mat);
        assert_eq!(
            cmb.cross_section_class(CrossSectionClassCase::WebCompression),
            CrossSectionClass::Three
        );
    }
}
