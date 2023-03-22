use crate::{
    angle::Angle,
    quaternion::{self, Quat, Quaternion},
    util::interpolation::soft_clamp,
};

pub type Vec3 = (f32, f32, f32);

pub const X: Vec3 = (1.0, 0.0, 0.0);
pub const Y: Vec3 = (0.0, 1.0, 0.0);
pub const Z: Vec3 = (0.0, 0.0, 1.0);

pub trait Vector3 {
    const ZERO: Vec3 = (0.0, 0.0, 0.0);

    fn magnitude_sq(&self) -> f32;

    fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    fn dot(self, other: Self) -> f32;
    fn cross(self, other: Vec3) -> Vec3;
    fn apply_rotation(self, r: Quaternion) -> Vec3;
    fn rotate(self, axis: Vec3, angle: Angle) -> Vec3;
    fn multiply_scalar(self, scalar: f32) -> Vec3;
    fn normalize(self) -> Vec3;
    fn rotate_xyz(self, other: Vec3) -> Vec3;
    fn rgb_u8(self) -> (u8, u8, u8);
    fn sub(self, rhs: Vec3) -> Vec3;
    fn add(self, rhs: Self) -> Vec3;
    fn channel_multiply(self, rhs: Vec3) -> Vec3;
    fn add_assign(&mut self, rhs: Vec3);
    fn sub_assign(&mut self, rhs: Vec3);
    fn max(self, max: f32) -> f32;
}

impl Vector3 for Vec3 {
    #[inline]
    fn magnitude_sq(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    #[inline]
    fn dot(self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    #[inline]
    fn cross(self, other: Vec3) -> Vec3 {
        (
            other.1 * self.2 - other.2 * self.1,
            other.2 * self.0 - other.0 * self.2,
            other.0 * self.1 - other.1 * self.0,
        )
    }

    fn apply_rotation(self, r: Quaternion) -> Vec3 {
        let r = r.inverse();
        let v = self;
        let (s, x, y, z) = r;
        let u = (x, y, z);

        let a = u.multiply_scalar(u.dot(v) * 2.0);
        let b = v.multiply_scalar((s * s) - u.dot(u));
        let c = u.cross(v).multiply_scalar(2.0 * s);

        a.add(b).add(c)
    }

    #[inline]
    fn rotate(self, axis: Vec3, angle: Angle) -> Vec3 {
        let r = quaternion::get_rotation(angle, axis);
        self.apply_rotation(r)
    }

    #[inline]
    fn multiply_scalar(self, scalar: f32) -> Vec3 {
        (self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }

    #[inline]
    fn normalize(self) -> Vec3 {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.multiply_scalar(1.0 / mag)
        } else {
            self
        }
    }

    fn channel_multiply(self, rhs: Vec3) -> Vec3 {
        let (x1, y1, z1) = self;
        let (x2, y2, z2) = rhs;
        (x1 * x2, y1 * y2, z1 * z2)
    }

    fn rotate_xyz(self, other: Vec3) -> Vec3 {
        let (x, y, z) = other;
        self.rotate((1.0, 0.0, 0.0), Angle::from_radians(x))
            .rotate((0.0, 1.0, 0.0), Angle::from_radians(y))
            .rotate((0.0, 0.0, 1.0), Angle::from_radians(z))
    }

    fn rgb_u8(self) -> (u8, u8, u8) {
        let (x, y, z) = self;
        (
            soft_clamp(x * 255.0, 0.0, 255.0) as u8,
            soft_clamp(y * 255.0, 0.0, 255.0) as u8,
            soft_clamp(z * 255.0, 0.0, 255.0) as u8,
        )
    }

    fn sub(self, rhs: Vec3) -> Vec3 {
        (self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }

    fn add(self, rhs: Vec3) -> Vec3 {
        (self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }

    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }

    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }

    fn max(self, max: f32) -> f32 {
        self.0.max(self.1).max(self.2).max(max)
    }
}
