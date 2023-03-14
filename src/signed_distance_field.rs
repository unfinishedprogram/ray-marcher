mod combine;
mod plane;
mod sphere;

pub use combine::*;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::vector3::Vector3;

pub trait SignedDistance: Sync {
    fn distance_from(&self, position: Vector3) -> f64;
}
