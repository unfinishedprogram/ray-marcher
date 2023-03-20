use crate::vector3::Vec3;

use super::SignedDistance;

pub struct Rounded<T: SignedDistance + Sized>(pub Box<T>, pub f64);

impl<T: SignedDistance + Sized> SignedDistance for Rounded<T> {
    fn distance_from(&self, point: Vec3) -> f64 {
        self.0.distance_from(point) - self.1
    }
}
