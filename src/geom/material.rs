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
    fuzz: Option<f64>
}

impl Metal {

    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        let albedo = Colour::new(r, g, b);
        let fuzz = Some(fuzz);
        Metal {
            albedo, fuzz
        }
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * *n
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut reflected = Metal::reflect(&r_in.direction.unit_vector(), &rec.normal);
        if let Some(fuzz) = self.fuzz {
            reflected = reflected + fuzz * Vec3::random_in_unit_sphere();
        }

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

pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ir: index_of_refraction
        }
    }

    fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta =  (-*uv).dot(n).min(1.0);
        let r_out_perp =  (*uv + cos_theta * *n) * etai_over_etat;
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if rec.front_face  { (1.0/self.ir) } else { self.ir};

        let unit_direction = r_in.direction.unit_vector();
        let refracted = Dielectric::refract(&unit_direction, &rec.normal, refraction_ratio);

        let scattered = Ray::new(rec.p, refracted);
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        Some(Scatter {
                scattered,
                attenuation
            })

    }
}