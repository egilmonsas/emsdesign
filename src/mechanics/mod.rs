use std::ops::{Div, Mul, Sub};
#[derive(PartialEq, Debug, Clone, Copy)]

pub struct LineLoad {
    pub magnitude: f64,
    pub direction: Vec2,
}
impl LineLoad {
    pub fn new(direction: Vec2, magnitude: f64) -> Self {
        Self {
            direction: direction.normalize(),
            magnitude,
        }
    }
    pub fn compute_field_moment(&self, parent: &Member, length_scalar: f64) -> f64 {
        let q_normal = self.direction.decompose(&parent.vector()).1.magnitude() * self.magnitude;

        let length = parent.length();
        let x = length_scalar * length;
        let moment_at_x = 0.5 * length_scalar * length.powi(2) * q_normal * (1.0 - length_scalar);

        moment_at_x
    }
}

pub struct Member {
    start_point: Vec2,
    end_point: Vec2,
    loads: Vec<LineLoad>,
}

impl Member {
    pub const fn new(start_point: Vec2, end_point: Vec2) -> Self {
        Self {
            start_point,
            end_point,
            loads: vec![],
        }
    }

    pub fn length(&self) -> f64 {
        self.vector().magnitude()
    }
    pub fn angle(&self) -> f64 {
        let v = self.vector();
        f64::atan2(v.y, v.x)
    }
    pub fn vector(&self) -> Vec2 {
        self.end_point - self.start_point
    }
    pub fn point_along_length(&self, scalar: f64) -> Vec2 {
        let scalar = scalar.clamp(0.0, 1.0);
        self.vector() * scalar
    }
    #[must_use]
    pub fn apply_line_load(mut self, line_load: LineLoad) -> Self {
        self.loads.push(line_load);
        self
    }
    pub fn compute_moment(&self, length_scalar: f64) -> f64 {
        self.loads.iter().fold(0.0, |mom_sum, load| {
            mom_sum + load.compute_field_moment(self, length_scalar)
        })
    }
}

impl Default for Member {
    fn default() -> Self {
        Self::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0))
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    #[must_use]
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn project(&self, vector_to_project_unto: &Self) -> Self {
        let dot = self.dot(vector_to_project_unto);
        let mag = vector_to_project_unto.magnitude();
        *vector_to_project_unto * (dot / mag.powi(2))
    }
    pub fn decompose(self, vector_to_decompose_onto: &Self) -> (Self, Self) {
        let parallell_component = self.project(vector_to_decompose_onto);
        let perpendicular_compontent = self - parallell_component;
        (parallell_component, perpendicular_compontent)
    }
    pub fn normalize(mut self) -> Self {
        self = self / self.magnitude();
        self
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::Zeq;
    #[test]
    fn can_create_member() {
        let member = Member::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        assert_eq!(member.start_point, Vec2::new(0.0, 0.0));
        assert_eq!(member.end_point, Vec2::new(0.0, 1.0));
    }
    #[test]
    fn member_computes_length_of_unit_member_correctly() {
        let member = Member::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        assert_zeq!(member.length(), 1.0);
    }
    #[test]
    fn member_computes_angle_of_unit_member_correctly() {
        let pi8 = std::f64::consts::FRAC_PI_8;
        let member = Member::new(Vec2::new(0.0, 0.0), Vec2::new(pi8.cos(), pi8.sin()));
        assert_zeq!(member.angle(), pi8);
    }
    #[test]
    fn point_along_length_works() {
        let member = Member::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        dbg!(member.point_along_length(0.3));
    }
    #[test]
    fn correct_self_weight_moment() {
        let member = Member::new(Vec2::new(0.0, 0.0), Vec2::new(20.0, 0.0))
            .apply_line_load(LineLoad::new(Vec2::new(0.0, 1.0), 1.0));
        dbg!(member.compute_moment(0.5));
    }
}
