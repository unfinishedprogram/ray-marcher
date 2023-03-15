mod pbr;

use crate::vector3::Vec3;
pub use pbr::PBRMaterial;

pub enum Material {
    Basic(Vec3),
}
