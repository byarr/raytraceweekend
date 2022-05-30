use std::io::stdout;
use raytraceweekend::{Colour, Point3, Ray, Sphere, Vec3, write_png};
use raytraceweekend::geom::shape::{Hittable, HittableList};

fn ray_colour<H: Hittable>(r: &Ray, hittable: &H) -> Colour {

    let hit = hittable.hit(r, 0.0, f64::INFINITY);

    if let Some(t) = hit {
        return 0.5 * (t.normal + Colour::new(1.0,1.0,1.0));
    }

    let unit = r.direction.unit_vector();
    let t = 0.5 * (unit.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList { objects: Vec::new() };
    world.objects.push(Box::new(Sphere{ center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 }));
    world.objects.push(Box::new(Sphere{ center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 }));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut result = Vec::new();
    for j in (0..image_height).rev() {
        eprintln!("Scan lines remaining {j}");
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_colour(&r, &world);
            result.push(pixel_color);
            // println!("{pixel_color}");
        }
    }

    write_png(&mut stdout(), &result, image_width as u32, image_height as u32);
    eprintln!("Done!")
}
