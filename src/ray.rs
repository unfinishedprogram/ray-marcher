use crate::vector3::Vector3;

pub struct ViewRay {
    pub origin: Vector3,
    pub position: Vector3,
    pub orientation: Vector3,
    pub steps: u32,
    pub color: (u8, u8, u8),
}

impl ViewRay {
    pub fn new(origin: impl Into<Vector3>, orientation: impl Into<Vector3>) -> Self {
        let origin = origin.into();

        Self {
            origin,
            position: origin,
            orientation: orientation.into(),
            steps: 0,
            color: (0, 0, 0),
        }
    }

    #[inline]
    pub fn step(&mut self, distance: f32) {
        self.steps += 1;
        self.position += self.orientation.multiply_scalar(distance);
    }
}
