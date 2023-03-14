use crate::vector3::Vec3;

use super::Material;

pub struct PBRMaterial {
    roughness: f64,
    specular: f64,
    metallic: f64,
    albedo: Vec3,
}

impl Material for PBRMaterial {}
