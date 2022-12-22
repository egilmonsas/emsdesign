use super::CrossSection;

use polars::prelude::*;


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PRESETS {
    HEB,
    CHS
}

impl PRESETS{
    pub fn path(&self)-> String {
        let prefix = "src/crs/data/";
        let suffix = ".csv";
        let filename = match self {
            PRESETS::HEB=> "HEB",
            PRESETS::CHS=> "CHS",
        };

        let mut buffer= String::new();
        buffer.push_str(prefix);
        buffer.push_str(filename);
        buffer.push_str(suffix);

        buffer
    }
}

pub struct CrsLib{
    df: LazyFrame
}

impl CrsLib{
    pub fn new(path:&str)->Self{
        let mut s = Schema::default();
        s.coerce_by_index( 0, DataType::Utf8);
        let file = std::fs::File::open(path).unwrap();
        let df = CsvReader::new(file)
            .with_delimiter(b',')
            .has_header(true)
            .with_dtypes(Some(&s))
            .finish();
        match df {
            Ok(df)=> return Self { df: df.lazy() },
            Err(_) => panic!("Couldnt read that shit")
        };
    }
}

pub struct PresetCrs{
    data: DataFrame
}

impl PresetCrs{
    pub fn new(label:&str, lib:&CrsLib)->Self{
        // HELLA TRASH FUNCTION, PLEASE FIX
        let mask = col("Section").eq(lit(label));
        let temp = lib.df.clone().filter(mask).collect();
        match temp {
            Ok(df)=> return Self { data:df },
            Err(_) => panic!("Couldnt read that shit")
        };
    }
}

impl CrossSection for PresetCrs{
    fn area(&self) -> f64 {
        // HELLA TRASH FUNCTION, PLEASE FIX
        self.data.column("A[cm2]").unwrap().sum::<f64>().unwrap()
    }

    fn centroid(&self) -> (f64, f64) {
        (
        self.data.column("d[mm]").unwrap().sum::<f64>().unwrap()/2.0,
        self.data.column("d[mm]").unwrap().sum::<f64>().unwrap()/2.0
        )
    }

    fn Iy(&self) -> f64 {
        self.data.column("Iy[cm4]").unwrap().sum::<f64>().unwrap()*10.0f64.powi(4) 
    }
    fn Iz(&self) -> f64 {
        self.data.column("Iz[cm4]").unwrap().sum::<f64>().unwrap()*10.0f64.powi(4) 
    }
    fn wy(&self) -> f64 {
        self.data.column("Wy[cm3]").unwrap().sum::<f64>().unwrap()*10.0f64.powi(3)      
    }
    fn wz(&self) -> f64 {
        self.data.column("Wz[cm3]").unwrap().sum::<f64>().unwrap()*10.0f64.powi(3)       
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zeq::Zeq;

    #[test]
    fn it_works(){
        let df = CrsLib::new(&PRESETS::CHS.path());
        let crs = PresetCrs::new("Celsius 355 CHS 323.9x8", &df);
        assert_zeq!(79.40,crs.area());
    }

}
