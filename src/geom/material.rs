use crate::geom::shape::HitRecord;
use crate::{Colour, Ray, Vec3};

struct Scatter {
    scattered: Ray,
    attenuation: Colour,
}

trait Material {
    fn scatter(&self, hit_record: &HitRecord) -> Option<Scatter>;
}

struct Lambertian {
    albedo: Colour,
}

impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord) -> Option<Scatter> {
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
