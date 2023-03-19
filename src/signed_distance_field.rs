mod combine;
mod primitive;
mod transform;

use crate::{quaternion::Quaternion, vector3::Vec3};
pub use combine::*;
pub use primitive::Primitive;
pub use transform::{Rotation, Translation};

pub trait SignedDistance: Sync {
    fn distance_from(&self, position: Vec3) -> f64;

    fn rotate(self, rotation: Quaternion) -> Rotation<Self>
    where
        Self: Sized,
    {
        Rotation(Box::new(self), rotation)
    }

    fn translate(self, translation: Vec3) -> Translation<Self>
    where
        Self: Sized,
    {
        Translation(Box::new(self), translation)
    }

    fn translate_x(self, x: f64) -> Translation<Self>
    where
        Self: Sized,
    {
        self.translate((x, 0.0, 0.0))
    }

    fn translate_y(self, y: f64) -> Translation<Self>
    where
        Self: Sized,
    {
        self.translate((0.0, y, 0.0))
    }

    fn translate_z(self, z: f64) -> Translation<Self>
    where
        Self: Sized,
    {
        self.translate((0.0, 0.0, z))
    }
}
