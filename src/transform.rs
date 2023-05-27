use crate::vector3::Vec3;

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(rotation: Vec3, translation: Vec3, scale: Vec3) -> Transform {
        Transform {
            translation,
            rotation,
            scale,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        }
    }
}

// translate -> rotate -> scale
// x -> y -> z
