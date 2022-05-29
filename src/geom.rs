use std::fmt::{Display, Formatter, write};
use std::ops::{AddAssign, DivAssign, Index, MulAssign};

#[derive(Debug, PartialOrd, PartialEq, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

pub type Colour = Vec3;
pub type Point3 = Vec3;

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let e = [x, y, z];
        Self {e}
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|i| i * i).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.e[i] += rhs.e[i];
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.e[i] *= rhs;
        }
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl Colour {
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = (self.r()  * 255.999) as i32;
        let g = (self.g()  * 255.999) as i32;
        let b = (self.b()  * 255.999) as i32;
        write!(f, "{r} {g} {b}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(50.0, v.length_squared());
        assert!(f64::abs(7.07 - v.length()) < 0.01);
    }

}