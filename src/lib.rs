mod camera;
pub mod geom;
mod png;

pub use geom::Colour;
pub use geom::Point3;
pub use geom::Ray;
pub use geom::Vec3;

pub use camera::Camera;

pub use crate::png::write_png;
pub use geom::shape::Sphere;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}
