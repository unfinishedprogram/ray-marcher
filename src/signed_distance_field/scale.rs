use crate::vector3::{Vec3, Vector3};

use super::SignedDistance;

pub struct Scaled<T: SignedDistance + Sized>(pub Box<T>, pub f32);

impl<T: SignedDistance + Sized> SignedDistance for Scaled<T> {
    fn distance_from(&self, point: Vec3) -> f32 {
        self.0.distance_from(point.multiply_scalar(self.1))
    }
}
