mod pbr;

use crate::vector3::Vec3;
pub use pbr::PBRMaterial;

#[derive(Clone)]
pub enum Material {
    Basic(Vec3),
}
