use crate::vector3::{Vec3, Vector3};

use super::SignedDistance;

pub struct Plane {
    normal: Vec3,
    position: Vec3,
}

impl Plane {
    pub fn new(normal: Vec3, position: Vec3) -> Self {
        Self {
            normal: normal.normalize(),
            position,
        }
    }
}

impl SignedDistance for Plane {
    fn distance_from(&self, position: Vec3) -> f64 {
        self.normal.dot(position.sub(self.position))
    }
}
