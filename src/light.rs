use crate::vector3::Vector3;

pub struct Light {
    pub position: Vector3,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: impl Into<Vector3>, intensity: f64) -> Self {
        Self {
            position: position.into(),
            intensity,
        }
    }
}
