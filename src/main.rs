use std::io::stdout;
use rand::prelude::*;
use rand::Rng;
use raytraceweekend::{Camera, Colour, Point3, Ray, Sphere, Vec3, write_png};
use raytraceweekend::geom::shape::{Hittable, HittableList};


fn ray_colour<H: Hittable>(r: &Ray, hittable: &H, depth: u32) -> Colour {

    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let hit = hittable.hit(r, 0.001, f64::INFINITY);

    if let Some(t) = hit {
        let target = t.p + t.normal + Vec3::random_in_unit_sphere().unit_vector();
        return 0.5 * ray_colour(&Ray::new(t.p, target - t.p), hittable, depth-1);
    }

    let unit = r.direction.unit_vector();
    let t = 0.5 * (unit.y() + 1.0);
    ((1.0 - t) * Colour::new(1.0, 1.0, 1.0)) + (t * Colour::new(0.5, 0.7, 1.0))
}

fn main() {

    let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEADBEEF);

    let args: Vec<_> = std::env::args().collect();
    let samples_per_pixel = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(1);

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_depth = 50;

    // World
    let mut world = HittableList { objects: Vec::new() };
    world.objects.push(Box::new(Sphere{ center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }));
    world.objects.push(Box::new(Sphere{ center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 }));

    // Camera
    let camera = Camera::default();

    let mut result = Vec::new();
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining {j}");
        for i in 0..image_width {
            let mut pixel_color = Colour::default();

            for k in 0..samples_per_pixel {
                let u = (i as f64 + random_double(&mut rng, samples_per_pixel)) / (image_width-1) as f64;
                let v = (j as f64 + random_double(&mut rng, samples_per_pixel)) / (image_height-1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_colour(&r, &world, max_depth);
            }
            result.push(pixel_color);
            // println!("{pixel_color}");
        }
    }

    write_png(&mut stdout(), &result, image_width as u32, image_height as u32, samples_per_pixel);
    eprintln!("Done!")
}

fn random_double(rng: &mut StdRng, samples_per_pixel: u32) -> f64 {
    return if samples_per_pixel == 1 {
        0.0
    } else {
        rng.gen()
    }
}