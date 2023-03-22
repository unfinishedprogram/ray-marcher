use crate::vector3::Vec3;

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub radius: f32,
}

impl Light {
    pub fn new(position: impl Into<Vec3>, color: impl Into<Vec3>, radius: f32) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            radius,
        }
    }
}
