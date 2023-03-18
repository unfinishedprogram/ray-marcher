use crate::{material::Material, signed_distance_field::SignedDistance, vector3::Vec3};

pub trait Entity: Sync {
    fn get_material(&self) -> &Material;
    fn distance(&self, point: Vec3) -> f64;
}

// An entity to rendered in a scene
pub struct BasicEntity<T: SignedDistance + 'static> {
    pub signed_distance: Box<T>,
    pub material: Material,
}

impl<T: SignedDistance + 'static> BasicEntity<T> {
    pub fn new(signed_distance: T, material: Material) -> Self {
        Self {
            signed_distance: Box::new(signed_distance),
            material,
        }
    }

    pub fn distance(&self, point: Vec3) -> f64 {
        self.signed_distance.distance_from(point)
    }
}

impl<T: SignedDistance> Entity for BasicEntity<T> {
    fn get_material(&self) -> &Material {
        &self.material
    }

    fn distance(&self, point: Vec3) -> f64 {
        self.distance(point)
    }
}
