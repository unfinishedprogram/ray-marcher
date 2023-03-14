use crate::vector3::{Vec3, Vector3};

use super::SignedDistance;

pub struct Sphere {
    position: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: impl Into<Vec3>, radius: f64) -> Self {
        Sphere {
            position: position.into(),
            radius,
        }
    }
}

impl SignedDistance for Sphere {
    fn distance_from(&self, position: Vec3) -> f64 {
        (position.sub(self.position)).magnitude() - self.radius
    }
}
