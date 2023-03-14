use crate::{
    angle::Angle,
    vector3::{Vec3, Vector3},
};

pub type Quaternion = (f64, f64, f64, f64);

pub fn get_rotation(angle: Angle, (x, y, z): Vec3) -> Quaternion {
    let (sin_a, cos_a) = (angle.rad() / 2.0).sin_cos();
    (cos_a, sin_a * x, sin_a * y, sin_a * z)
}

pub fn rotation_from_to(from: Vec3, to: Vec3) -> Quaternion {
    let (x, y, z) = to.cross(from);
    let w = (to.magnitude_sq() * from.magnitude_sq()).sqrt() + to.dot(from);
    normalize((w, x, y, z))
}

pub fn normalize((a, b, c, d): Quaternion) -> Quaternion {
    let magnitude = (a * a + b * b + c * c + d * d).sqrt();
    (a / magnitude, b / magnitude, c / magnitude, d / magnitude)
}

pub fn hamilton_product(a: Quaternion, b: Quaternion) -> Quaternion {
    (
        (a.0 * b.0 - a.1 * b.1 - a.2 * b.2 - a.3 * b.3),
        (a.0 * b.1 + a.1 * b.0 + a.2 * b.3 - a.3 * b.2),
        (a.0 * b.2 - a.1 * b.3 + a.2 * b.0 + a.3 * b.1),
        (a.0 * b.3 + a.1 * b.2 - a.2 * b.1 + a.3 * b.0),
    )
}