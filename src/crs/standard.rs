use super::CrossSection;
use std::io::prelude::*;

use polars::prelude::*;

const HEB: &'static [u8] = include_bytes!("./data/HEB.csv");
const CHS: &'static [u8] = include_bytes!("./data/CHS.csv");

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PRESETS {
    HEB,
    CHS,
}

impl PRESETS {
    pub fn is_symmetric(&self) -> bool {
        return match self {
            PRESETS::HEB => false,
            PRESETS::CHS => true,
        };
    }
    pub fn embeded_bytes(&self) -> &'static [u8] {
        return match self {
            PRESETS::HEB => HEB,
            PRESETS::CHS => CHS,
        };
    }
    pub fn path_str(&self) -> String {
        let prefix = "c:/WINDOWS/Temp/";
        let suffix = ".csv";
        let filename = match self {
            PRESETS::HEB => "HEB",
            PRESETS::CHS => "CHS",
        };

        let mut buffer = String::new();

        buffer.push_str(prefix);
        buffer.push_str(filename);
        buffer.push_str(suffix);

        buffer
    }
}
pub struct CrsLib {
    df: LazyFrame,
    is_symmetric: bool,
}

impl CrsLib {
    /// Shit function but here we go
    /// As far as my peanut brain can tell, there isnt an easy API in polars that allow you to create a
    /// Embeded csv bytes are written into a file that is built into the Temp directory
    /// The handle is then read back into polars
    pub fn new(presets: &PRESETS) -> Self {
        // Grab associated pathnames and bytes for a given type
        let path = presets.path_str();
        let bytes = presets.embeded_bytes();

        // Create and write the file
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(bytes);

        // Enforce stringtype on column 0
        let mut s = Schema::default();
        s.coerce_by_index(0, DataType::Utf8);

        // Create and return lazyframe
        let df = CsvReader::new(std::fs::File::open(&path).unwrap())
            .with_delimiter(b',')
            .has_header(true)
            .with_dtypes(Some(&s))
            .finish();
        match df {
            Ok(df) => {
                return Self {
                    df: df.lazy(),
                    is_symmetric: presets.is_symmetric(),
                }
            }
            Err(_) => panic!("Couldnt compile/read file"),
        };
    }
}

pub struct PresetCrs {
    data: DataFrame,
    is_symmetric: bool,
}

impl PresetCrs {
    pub fn new(label: &str, lib: &CrsLib) -> Self {
        // HELLA TRASH FUNCTION, PLEASE FIX
        let mask = col("Section").eq(lit(label));
        let temp = lib.df.clone().filter(mask).collect();
        match temp {
            Ok(df) => {
                return Self {
                    data: df,
                    is_symmetric: lib.is_symmetric,
                }
            }
            Err(_) => panic!("Couldnt read that shit"),
        };
    }

    fn is_symmetric(&self) -> bool {
        self.is_symmetric
    }
}

impl CrossSection for PresetCrs {
    fn area(&self) -> f64 {
        // HELLA TRASH FUNCTION, PLEASE FIX
        self.data.column("A[cm2]").unwrap().sum::<f64>().unwrap() * 10.0f64.powi(2)
    }

    fn centroid(&self) -> (f64, f64) {
        (
            self.data.column("d[mm]").unwrap().sum::<f64>().unwrap() / 2.0,
            self.data.column("d[mm]").unwrap().sum::<f64>().unwrap() / 2.0,
        )
    }

    fn Iy(&self) -> f64 {
        self.data.column("Iy[cm4]").unwrap().sum::<f64>().unwrap() * 10.0f64.powi(4)
    }
    fn Iz(&self) -> f64 {
        if self.is_symmetric() {
            self.Iy()
        } else {
            self.data.column("Iz[cm4]").unwrap().sum::<f64>().unwrap() * 10.0f64.powi(4)
        }
    }
    fn wy(&self) -> f64 {
        self.data.column("Wy[cm3]").unwrap().sum::<f64>().unwrap() * 10.0f64.powi(3)
    }
    fn wz(&self) -> f64 {
        if self.is_symmetric() {
            self.wy()
        } else {
            self.data.column("Wz[cm3]").unwrap().sum::<f64>().unwrap() * 10.0f64.powi(3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn it_works() {
        let df = CrsLib::new(&PRESETS::CHS);
        let crs = PresetCrs::new("Celsius 355 CHS 323.9x8", &df);
        assert_zeq!(7940.0, crs.area());
    }
}
