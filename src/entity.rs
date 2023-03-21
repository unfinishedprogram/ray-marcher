use crate::{material::Material, signed_distance_field::SignedDistance, vector3::Vec3};

// An entity to rendered in a scene
pub struct Entity {
    pub signed_distance: Box<dyn SignedDistance>,
    pub material: Material,
}

impl Entity {
    pub fn with_material(
        signed_distance: impl SignedDistance + 'static,
        material: Material,
    ) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            material,
        }
    }

    pub fn new(signed_distance: impl SignedDistance + 'static) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            material: Material {
                albedo: (1.0, 1.0, 1.0),
            },
        }
    }

    pub fn distance(&self, point: Vec3) -> f64 {
        self.signed_distance.distance_from(point)
    }
}
