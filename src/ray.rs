use crate::vector3::Vector3;

pub struct ViewRay {
    pub origin: Vector3,
    pub position: Vector3,
    pub orientation: Vector3,
    pub steps: u32,
    pub color: (u8, u8, u8),
    pub clip: (f64, f64),
}

impl ViewRay {
    pub fn new(origin: Vector3, orientation: Vector3, clip: (f64, f64)) -> Self {
        let origin = origin;

        Self {
            origin,
            position: origin,
            orientation,
            steps: 0,
            color: (0, 0, 0),
            clip,
        }
    }

    #[inline]
    pub fn len_sq(&self) -> f64 {
        (self.origin - self.position).magnitude_sq()
    }

    #[inline]
    pub fn step(&mut self, distance: f64) {
        self.steps += 1;
        self.position += self.orientation.multiply_scalar(distance);
    }
}
