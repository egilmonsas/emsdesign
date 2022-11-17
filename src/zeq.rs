const TOLERANCE: f64 = 0.0001;

pub trait Zeq<T> {
    fn zeq(&self, other: T) -> bool;

    fn zneg(&self, other: T) -> bool {
        !self.zeq(other)
    }
}

impl Zeq<f64> for f64 {
    fn zeq(&self, other: f64) -> bool {
        let epsilon = TOLERANCE;
        (*self - other).abs() < epsilon
    }
}

#[macro_export]
macro_rules! assert_zeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zneg(*right) {
                    panic!(
                        "asserting zequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_nzeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zeq(*right) {
                    panic!(
                        "asserting inzequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}
