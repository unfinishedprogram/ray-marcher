use crate::vector3::{Vec3, Vector3};

pub struct ViewRay {
    pub origin: Vec3,
    pub position: Vec3,
    pub orientation: Vec3,
    pub steps: u32,
    pub color: Vec3,
    pub clip: (f64, f64),
}

impl ViewRay {
    pub fn new(origin: Vec3, orientation: Vec3, clip: (f64, f64)) -> Self {
        let origin = origin;

        Self {
            origin,
            position: origin,
            orientation,
            steps: 0,
            color: (0.0, 0.0, 0.0),
            clip,
        }
    }

    #[inline]
    pub fn len_sq(&self) -> f64 {
        (self.origin.sub(self.position)).magnitude_sq()
    }

    #[inline]
    pub fn step(&mut self, distance: f64) {
        self.steps += 1;
        self.position
            .add_assign(self.orientation.multiply_scalar(distance));
    }
}
