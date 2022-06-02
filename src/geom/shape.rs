use crate::geom::material::Material;
use crate::{Point3, Ray, Vec3};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<Box<dyn Material>>,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        r: &Ray,
        material: Rc<Box<dyn Material>>,
    ) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        // bool hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let rec = object.hit(r, t_min, closest_so_far);
            if let Some(r) = rec {
                closest_so_far = r.t;
                temp_rec = Some(r);
            }
        }
        temp_rec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(
            p,
            outward_normal,
            t,
            r,
            self.material.clone(),
        ))
    }
}
