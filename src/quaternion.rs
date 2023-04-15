use crate::{
    angle::Angle,
    vector3::{Vec3, Vector3},
};

pub type Quaternion = (f32, f32, f32, f32);
pub trait Quat {
    fn magnitude_sq(self) -> f32;
    fn inverse(self) -> Quaternion;
    fn multiply_scalar(self, scalar: f32) -> Quaternion;
    fn is_identity(&self) -> bool;
}

impl Quat for Quaternion {
    fn is_identity(&self) -> bool {
        let (a, b, c, d) = *self;
        a == 1.0 && b == 0.0 && c == 0.0 && d == 0.0
    }

    fn magnitude_sq(self) -> f32 {
        let (a, b, c, d) = self;
        a * a + b * b + c * c + d * d
    }

    fn inverse(self) -> Quaternion {
        let (a, b, c, d) = self;
        let conjugate = (a, -b, -c, -d);
        let magnitude = self.magnitude_sq();
        conjugate.multiply_scalar(1.0 / magnitude)
    }

    fn multiply_scalar(self, scalar: f32) -> Quaternion {
        (
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
            self.3 * scalar,
        )
    }
}

pub fn identity_quaternion() -> Quaternion {
    (1.0, 0.0, 0.0, 0.0)
}

pub fn get_rotation(angle: Angle, (x, y, z): Vec3) -> Quaternion {
    let (sin_a, cos_a) = (angle.rad() / 2.0).sin_cos();
    (cos_a, sin_a * x, sin_a * y, sin_a * z)
}

pub fn rotation_from_to(from: Vec3, to: Vec3) -> Quaternion {
    let (x, y, z) = to.cross(from);
    let w = (to.magnitude_sq() * from.magnitude_sq()).sqrt() + to.dot(from);
    normalize((x, y, z, w))
}

pub fn normalize((a, b, c, d): Quaternion) -> Quaternion {
    let magnitude = (a * a + b * b + c * c + d * d).sqrt();
    let mul = 1.0 / magnitude;
    (a * mul, b * mul, c * mul, d * mul)
}

pub fn multiply(a: Quaternion, b: Quaternion) -> Quaternion {
    (
        a.0 * b.0 - a.1 * b.1 - a.2 * b.2 - a.3 * b.3,
        a.0 * b.1 + a.1 * b.0 + a.2 * b.3 - a.3 * b.2,
        a.0 * b.2 - a.1 * b.3 + a.2 * b.0 + a.3 * b.1,
        a.0 * b.3 + a.1 * b.2 - a.2 * b.1 + a.3 * b.0,
    )
}
