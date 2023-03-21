mod combine;
mod primitive;
pub mod repeat;
mod rouding;
mod scale;
mod transform;

use crate::{quaternion::Quaternion, vector3::Vec3};
pub use combine::*;
pub use primitive::Primitive;
pub use transform::{Rotation, Translation};

use self::{repeat::Repeated, rouding::Rounded, scale::Scaled};

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

    fn round(self, radius: f64) -> Rounded<Self>
    where
        Self: Sized,
    {
        Rounded(Box::new(self), radius)
    }

    fn repeat(self, interval: f64) -> Repeated<Self>
    where
        Self: Sized,
    {
        Repeated(Box::new(self), interval)
    }

    fn scale(self, factor: f64) -> Scaled<Self>
    where
        Self: Sized,
    {
        Scaled(Box::new(self), factor)
    }
}
