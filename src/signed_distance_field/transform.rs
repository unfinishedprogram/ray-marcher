use crate::{
    quaternion::{unit_quaternion, Quaternion},
    vector3::{Vec3, Vector3},
};

use super::SignedDistance;

pub struct Transform<T: SignedDistance> {
    pub rotation: Quaternion,
    pub translation: Vec3,
    pub signed_distance: Box<T>,
}

impl<T: SignedDistance> Transform<T> {
    pub fn new(signed_distance: T, translation: Vec3, rotation: Quaternion) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            rotation,
            translation,
        }
    }

    pub fn translate(signed_distance: T, translation: Vec3) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            rotation: unit_quaternion(),
            translation,
        }
    }

    pub fn rotate(signed_distance: T, rotation: Quaternion) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            rotation,
            translation: (0.0, 0.0, 0.0),
        }
    }
}

impl<T: SignedDistance> SignedDistance for Transform<T> {
    fn distance_from(&self, point: Vec3) -> f64 {
        self.signed_distance
            .distance_from(point.sub(self.translation).apply_rotation(self.rotation))
    }
}
