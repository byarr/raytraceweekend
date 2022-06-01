pub mod material;
pub mod shape;

use crate::clamp;
use rand::{thread_rng, Rng};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Range, Sub};

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
        self.e.iter().zip(other.e.iter()).map(|x| x.0 * x.1).sum()
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|i| i * i).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        *self / length
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
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
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
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

    pub fn write_color(&self, samples_per_pixel: u32, data: &mut Vec<u8>) {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        // Divide the color by the number of samples.
        let scale = 1.0 / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        data.push((256.0 * clamp(r, 0.0, 0.999)) as u8);
        data.push((256.0 * clamp(g, 0.0, 0.999)) as u8);
        data.push((256.0 * clamp(b, 0.0, 0.999)) as u8);
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
