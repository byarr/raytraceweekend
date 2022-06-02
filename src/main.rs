use rand::prelude::*;
use rand::Rng;
use raytraceweekend::geom::material::{Dielectric, Lambertian, Material, Metal};
use raytraceweekend::geom::shape::{Hittable, HittableList};
use raytraceweekend::{write_png, Camera, Colour, Point3, Ray, Sphere, Vec3};

use std::io::stdout;
use std::rc::Rc;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material: Rc<Box<dyn Material>> = Rc::new(Box::new(Lambertian::new(0.5, 0.5, 0.5)));

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = thread_rng().gen();
            let center = Point3::new(
                a as f64 + 0.9 * thread_rng().gen::<f64>(),
                0.2,
                b as f64 + 0.9 * thread_rng().gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Rc<Box<dyn Material>> = if choose_mat < 0.8 {
                    let albedo =
                        Colour::new(thread_rng().gen(), thread_rng().gen(), thread_rng().gen())
                            * Colour::new(
                                thread_rng().gen(),
                                thread_rng().gen(),
                                thread_rng().gen(),
                            );
                    Rc::new(Box::new(Lambertian { albedo }))
                } else if choose_mat < 0.95 {
                    let fuzz: f64 = thread_rng().gen_range(0.0..0.5);
                    Rc::new(Box::new(Metal::new(
                        thread_rng().gen_range(0.5..1.0),
                        thread_rng().gen_range(0.5..1.0),
                        thread_rng().gen_range(0.5..1.0),
                        fuzz,
                    )))
                } else {
                    Rc::new(Box::new(Dielectric::new(1.5)))
                };

                world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: material.clone(),
                }))
            }
        }
    }

    let material1: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1.clone(),
    }));

    let material2: Rc<Box<dyn Material>> = Rc::new(Box::new(Lambertian {
        albedo: Colour::new(0.4, 0.2, 0.1),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2.clone(),
    }));

    let material3: Rc<Box<dyn Material>> = Rc::new(Box::new(Metal::new(0.7, 0.6, 0.5, 0.0)));
    world.add(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3.clone(),
    }));

    world
}

fn ray_colour<H: Hittable>(r: &Ray, hittable: &H, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let hit = hittable.hit(r, 0.001, f64::INFINITY);

    if let Some(t) = hit {
        if let Some(scatter) = t.material.scatter(r, &t) {
            return scatter.attenuation * ray_colour(&scatter.scattered, hittable, depth - 1);
        } else {
            return Colour::new(0.0, 0.0, 0.0);
        }
    }

    let unit = r.direction.unit_vector();
    let t = 0.5 * (unit.y() + 1.0);
    ((1.0 - t) * Colour::new(1.0, 1.0, 1.0)) + (t * Colour::new(0.5, 0.7, 1.0))
}

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEADBEEF);

    let args: Vec<_> = std::env::args().collect();
    let samples_per_pixel = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(50);

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_depth = 50;

    // World

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Camera
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let mut result = Vec::new();
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining {j}");
        for i in 0..image_width {
            let mut pixel_color = Colour::default();

            for _k in 0..samples_per_pixel {
                let u = (i as f64 + random_double(&mut rng, samples_per_pixel))
                    / (image_width - 1) as f64;
                let v = (j as f64 + random_double(&mut rng, samples_per_pixel))
                    / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_colour(&r, &world, max_depth);
            }
            result.push(pixel_color);
            // println!("{pixel_color}");
        }
    }

    write_png(
        &mut stdout(),
        &result,
        image_width as u32,
        image_height as u32,
        samples_per_pixel,
    )
    .unwrap();
    eprintln!("Done!")
}

fn random_double(rng: &mut StdRng, samples_per_pixel: u32) -> f64 {
    if samples_per_pixel == 1 {
        0.0
    } else {
        rng.gen()
    }
}
