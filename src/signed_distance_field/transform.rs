use crate::{
    quaternion::{identity_quaternion, Quat, Quaternion},
    vector3::{Vec3, Vector3},
};

use super::SignedDistance;

pub struct Transform<T: SignedDistance> {
    pub rotation: Quaternion,
    inverse_rotation: Quaternion,
    pub translation: Vec3,
    pub signed_distance: Box<T>,
}

impl<T: SignedDistance> Transform<T> {
    pub fn new(signed_distance: T, translation: Vec3, rotation: Quaternion) -> Self {
        let inverse_rotation = rotation.inverse();
        Self {
            signed_distance: Box::new(signed_distance),
            rotation,
            translation,
            inverse_rotation,
        }
    }

    pub fn translate(signed_distance: T, translation: Vec3) -> Self {
        let inverse_rotation = identity_quaternion().inverse();
        Self {
            signed_distance: Box::new(signed_distance),
            rotation: identity_quaternion(),
            translation,
            inverse_rotation,
        }
    }

    pub fn rotate(signed_distance: T, rotation: Quaternion) -> Self {
        let inverse_rotation = rotation.inverse();
        Self {
            signed_distance: Box::new(signed_distance),
            rotation,
            translation: (0.0, 0.0, 0.0),
            inverse_rotation,
        }
    }
}

impl<T: SignedDistance> SignedDistance for Transform<T> {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        self.signed_distance.distance_from(
            point
                .sub(self.translation)
                .apply_rotation(self.inverse_rotation),
        )
    }
}
