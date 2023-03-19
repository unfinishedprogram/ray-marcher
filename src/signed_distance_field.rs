mod combine;
mod primitive;
mod transform;

use crate::vector3::Vec3;
pub use combine::*;
pub use primitive::Primitive;
pub use transform::Transform;

pub trait SignedDistance: Sync {
    fn distance_from(&self, position: Vec3) -> f64;
}
