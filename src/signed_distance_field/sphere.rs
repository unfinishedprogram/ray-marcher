use super::SignedDistance;
use crate::vector3::{Vec3, Vector3};

pub struct Sphere(pub f64);

impl SignedDistance for Sphere {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        point.magnitude() - self.0
    }
}
