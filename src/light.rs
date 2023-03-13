use crate::vector3::Vector3;

pub struct Light {
    pub position: Vector3,
    pub color: Vector3,
}

impl Light {
    pub fn new(position: impl Into<Vector3>, color: impl Into<Vector3>) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
        }
    }
}
