use crate::{
    quaternion::{Quat, Quaternion},
    vector3::{Vec3, Vector3},
};

use super::SignedDistance;
#[derive(Clone)]
pub struct Rotation<T: SignedDistance + Sized>(pub Box<T>, pub Quaternion);

#[derive(Clone)]
pub struct Translation<T: SignedDistance>(pub Box<T>, pub Vec3);

impl<T: SignedDistance> SignedDistance for Rotation<T> {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        self.0.distance_from(point.apply_rotation(self.1.inverse()))
    }
}

impl<T: SignedDistance> SignedDistance for Translation<T> {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        self.0.distance_from(point.sub(self.1))
    }
}
