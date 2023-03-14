use crate::vector3::Vector3;

use super::SignedDistance;

pub struct Plane {
    normal: Vector3,
    position: Vector3,
}

impl Plane {
    pub fn new(normal: impl Into<Vector3>, position: impl Into<Vector3>) -> Self {
        Self {
            normal: normal.into().normalize(),
            position: position.into(),
        }
    }
}

impl SignedDistance for Plane {
    fn distance_from(&self, position: Vector3) -> f64 {
        self.normal.dot(position - self.position)
    }
}
