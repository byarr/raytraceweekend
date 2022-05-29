pub mod shape;

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub};

#[derive(Debug, PartialOrd, PartialEq, Default, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Colour = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let e = [x, y, z];
        Self { e }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.e.iter().zip(other.e.iter()).map(|x| x.0 * x.1 ).sum()
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|i| i * i).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        self.clone() / length
    }

    pub fn x(&self) -> f64 {
        return self.e[0]
    }
    pub fn y(&self) -> f64 {
        return self.e[1]
    }
    pub fn z(&self) -> f64 {
        return self.e[2]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.e[i] += rhs.e[i];
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}



impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
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

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x()/rhs, self.y()/rhs, self.z()/rhs)
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
        let r = (self.r() * 255.999) as i32;
        let g = (self.g() * 255.999) as i32;
        let b = (self.b() * 255.999) as i32;
        write!(f, "{r} {g} {b}")
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
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

    #[test]
    fn test_ray_at() {
        let d = Vec3::new(3.0, 4.0, 5.0);
        let o = Point3::new(2.0, 3.0, 4.0);
        let ray = Ray::new(o, d);

        let vec3 = ray.at(3.5);
        assert_eq!(12.5, vec3.e[0]);
        assert_eq!(17.0, vec3.e[1]);
        assert_eq!(21.5, vec3.e[2]);
    }
}
