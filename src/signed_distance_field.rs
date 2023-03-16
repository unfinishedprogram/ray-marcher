mod combine;
mod plane;
mod sphere;
mod torus;
mod transform;

use crate::vector3::Vec3;
pub use combine::*;
pub use plane::Plane;
pub use sphere::Sphere;
pub use torus::Torus;
pub use transform::Transform;

pub trait SignedDistance: Sync {
    fn distance_from(&self, position: Vec3) -> f64;
}
