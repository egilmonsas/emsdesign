use super::CrossSection;
use crate::{err::EmsError, Axis};
use polars::prelude::*;
use std::io::prelude::*;

const HEB: &[u8] = include_bytes!("./data/HEB.csv");
const CHS: &[u8] = include_bytes!("./data/CHS.csv");

#[allow(dead_code)]
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
    #[must_use]
    pub const fn is_symmetric(&self) -> bool {
        match self {
            Self::HEB => false,
            Self::CHS => true,
        }
    }
    #[must_use]
    pub const fn embeded_bytes(&self) -> &'static [u8] {
        match self {
            Self::HEB => HEB,
            Self::CHS => CHS,
        }
    }
    #[must_use]
    pub fn path_str(&self) -> String {
        let prefix = "c:/WINDOWS/Temp/";
        let suffix = ".csv";
        let filename = match self {
            Self::HEB => "HEB",
            Self::CHS => "CHS",
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
    ///
    /// Super low hanging fruit in terms of optimization for instance only create this file once per launch of the app?
    ///
    /// # Errors
    ///
    /// Will return an error if something goes wrong with the file write/read
    #[allow(clippy::needless_return)]
    pub fn new(presets: &PRESETS) -> Result<Self, EmsError> {
        // Grab associated pathnames and bytes for a given type
        let path = presets.path_str();
        let bytes = presets.embeded_bytes();

        // Create and write the file
        let mut file = std::fs::File::create(&path).map_err(|e| {
            EmsError::write_error(
                format!("Could not create file at path: {}", &path),
                Some(Box::new(e)),
            )
        })?;
        file.write_all(bytes).map_err(|e| {
            EmsError::write_error(
                format!("Could not write to file at path: {}", &path),
                Some(Box::new(e)),
            )
        })?;

        // Enforce stringtype on column 0
        let mut s = Schema::default();
        s.coerce_by_index(0, DataType::Utf8);

        // Read file and return lazyframe
        let df = CsvReader::new(std::fs::File::open(&path).map_err(|e| {
            EmsError::file_not_found_error(
                format!("Could not read file at path: {}", &path),
                Some(Box::new(e)),
            )
        })?)
        .with_delimiter(b',')
        .has_header(true)
        .with_dtypes(Some(&s))
        .finish()
        .map_err(|e| {
            EmsError::file_not_found_error(
                format!("Could not read file at path: {}", &path),
                Some(Box::new(e)),
            )
        })?;
        Ok(Self {
            df: df.lazy(),
            is_symmetric: presets.is_symmetric(),
        })
    }

    /// # Errors
    /// Will return error if something is wrong with the polars dataframe navigation or reading
    pub fn sections(&self) -> Result<Vec<String>, EmsError> {
        let df = self.df.clone().collect().map_err(|e| {
            EmsError::database_error("Could not clone self.df".to_owned(), Some(Box::new(e)))
        })?;

        let series: Vec<String> = df
            .column("Section")
            .map_err(|e| {
                EmsError::database_error(
                    "Could not find the column named \"Sections\"".to_owned(),
                    Some(Box::new(e)),
                )
            })?
            .utf8()
            .map_err(|e| {
                EmsError::database_error(
                    "Could not convert into utf8".to_owned(),
                    Some(Box::new(e)),
                )
            })?
            .into_no_null_iter()
            .map(String::from)
            .collect();
        Ok(series)
    }
}

pub struct PresetCrs {
    width: f64,
    height: f64,
    area: f64,
    area_shear_y: f64,
    area_shear_z: f64,
    inertia_y: f64,
    w_elastic_y: f64,
    w_plastic_y: f64,
    inertia_z: f64,
    w_elastic_z: f64,
    w_plastic_z: f64,
}

impl PresetCrs {
    /// # Errors
    /// Will return an error if it cannot clone the lazyframe with mask
    pub fn new(label: &str, lib: &CrsLib) -> Result<Self, EmsError> {
        // HELLA TRASH FUNCTION, PLEASE FIX
        let mask = col("Section").eq(lit(label));
        let temp = lib.df.clone().filter(mask).collect().map_err(|e| {
            EmsError::database_error("Could not clone self.df".to_owned(), Some(Box::new(e)))
        })?;

        let out = if lib.is_symmetric {
            Self {
                area: Self::read_value(&temp, "A[cm2]")? * 1e2,
                area_shear_y: Self::read_value(&temp, "Ay[cm2]")? * 1e2,
                area_shear_z: Self::read_value(&temp, "Ay[cm2]")? * 1e2,
                width: Self::read_value(&temp, "d[mm]")?,
                height: Self::read_value(&temp, "d[mm]")?,
                inertia_y: Self::read_value(&temp, "Iy[cm4]")? * 1e4,
                w_elastic_y: Self::read_value(&temp, "Wy[cm3]")? * 1e3,
                w_plastic_y: Self::read_value(&temp, "Wpl,y[cm3]")? * 1e3,
                inertia_z: Self::read_value(&temp, "Iy[cm4]")? * 1e4,
                w_elastic_z: Self::read_value(&temp, "Wy[cm3]")? * 1e3,
                w_plastic_z: Self::read_value(&temp, "Wpl,y[cm3]")? * 1e3,
            }
        } else {
            Self {
                area: Self::read_value(&temp, "A[cm2]")? * 1e2,
                area_shear_y: Self::read_value(&temp, "Ay[cm2]")? * 1e2,
                area_shear_z: Self::read_value(&temp, "Az[cm2]")? * 1e2,
                width: Self::read_value(&temp, "d[mm]")?,
                height: Self::read_value(&temp, "d[mm]")?,
                inertia_y: Self::read_value(&temp, "Iy[cm4]")? * 1e4,
                w_elastic_y: Self::read_value(&temp, "Wy[cm3]")? * 1e3,
                w_plastic_y: Self::read_value(&temp, "Wpl,y[cm3]")? * 1e3,
                inertia_z: Self::read_value(&temp, "Iz[cm4]")? * 1e4,
                w_elastic_z: Self::read_value(&temp, "Wz[cm3]")? * 1e3,
                w_plastic_z: Self::read_value(&temp, "Wpl,z[cm3]")? * 1e3,
            }
        };
        Ok(out)
    }

    fn read_value(data: &DataFrame, key: &str) -> Result<f64, EmsError> {
        data.column(key)
            .map_err(|e| {
                EmsError::database_error(format!("Could not get column {}", key), Some(Box::new(e)))
            })?
            .sum::<f64>()
            .ok_or_else(|| {
                EmsError::database_error(format!("Could not get value in column {}", key), None)
            })
    }
}

impl CrossSection for PresetCrs {
    fn width(&self) -> f64 {
        self.width
    }
    fn height(&self) -> f64 {
        self.height
    }
    fn area(&self) -> f64 {
        self.area * 1e2
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;

    #[test]
    fn it_works() {
        let df = CrsLib::new(&PRESETS::CHS);
        let crs = PresetCrs::new("Celsius 355 CHS 323.9x8", &df.expect("Couldnt create df"))
            .expect("Couldnt create crs");
        assert_zeq!(794_000.0, crs.area());
    }

    #[test]
    fn can_collect_vector_from_section_names() {
        let df = CrsLib::new(&PRESETS::CHS).expect("Couldnt create df");
        let res = df.sections().expect("Couldnt get the sections");
        dbg!(res);
    }
}
