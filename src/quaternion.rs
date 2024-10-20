use crate::{angle::Angle, vector3::Vec3};

pub type Quaternion = (f32, f32, f32, f32);

pub fn get_rotation(angle: Angle, (x, y, z): Vec3) -> Quaternion {
    let (sin_a, cos_a) = (angle.rad() / 2.0).sin_cos();
    (cos_a, sin_a * x, sin_a * y, sin_a * z)
}
