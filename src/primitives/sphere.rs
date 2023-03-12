use crate::vector3::Vector3;

use super::SignedDistance;

pub struct Sphere {
    position: Vector3,
    radius: f32,
}

impl Sphere {
    pub fn new(position: impl Into<Vector3>, radius: f32) -> Self {
        Sphere {
            position: position.into(),
            radius,
        }
    }
}

impl SignedDistance for Sphere {
    fn distance_from(&self, position: Vector3) -> f32 {
        (position - self.position).magnitude() - self.radius
    }
}
