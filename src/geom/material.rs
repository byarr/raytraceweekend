use crate::geom::shape::HitRecord;
use crate::{Colour, Ray, Vec3};

#[derive(Debug)]
pub struct Scatter {
    pub scattered: Ray,
    pub attenuation: Colour,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some(Scatter {
            scattered,
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Colour,
}

impl Metal {
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = Metal::reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(Scatter {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
