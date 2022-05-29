use crate::{Point3, Ray};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, r: &Ray) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}