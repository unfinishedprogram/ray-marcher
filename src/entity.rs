use crate::{signed_distance_field::SignedDistance, vector3::Vec3};

// An entity to rendered in a scene
pub struct Entity {
    pub signed_distance: Box<dyn SignedDistance>,
}

impl Entity {
    pub fn with_material(signed_distance: impl SignedDistance + 'static) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
        }
    }

    pub fn new(signed_distance: impl SignedDistance + 'static) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
        }
    }

    pub fn distance(&self, point: Vec3) -> f32 {
        self.signed_distance.distance_from(point)
    }
}
