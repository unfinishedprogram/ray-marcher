pub mod plane;
pub mod sphere;

use crate::vector3::Vector3;

pub trait SignedDistance {
    fn distance_from(&self, position: Vector3) -> f32;
}
