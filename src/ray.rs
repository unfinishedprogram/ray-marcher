use crate::vector3::Vector3;

pub struct ViewRay {
    pub origin: Vector3,
    pub position: Vector3,
    pub orientation: Vector3,
    pub steps: u32,
    pub color: Vector3,
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
            color: Vector3::ZERO,
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
