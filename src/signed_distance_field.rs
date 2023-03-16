mod combine;
mod plane;
mod sphere;
mod transform;

pub use combine::*;
pub use plane::Plane;
pub use sphere::Sphere;
pub use transform::Transform;

use crate::vector3::Vec3;

pub trait SignedDistance: Sync {
    fn distance_from(&self, position: Vec3) -> f64;
}
