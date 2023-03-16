use crate::vector3::Vec3;

use super::SignedDistance;

pub struct Plane;

impl SignedDistance for Plane {
    fn distance_from(&self, position: Vec3) -> f64 {
        position.1
    }
}
