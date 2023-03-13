use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
    angle::Angle,
    quaternion::{self, hamilton_product, Quaternion},
};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[inline]
    pub fn magnitude_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.magnitude_sq().sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Vector3) -> Vector3 {
        (
            other.y * self.z - other.z * self.y,
            other.z * self.x - other.x * self.z,
            other.x * self.y - other.y * self.x,
        )
            .into()
    }

    pub fn apply_rotation(self, r: Quaternion) -> Vector3 {
        let Vector3 { x, y, z } = self;
        let p = (0.0, x, y, z);
        let r_prime = (r.0, -r.1, -r.2, -r.3);

        let (_, x, y, z) = hamilton_product(hamilton_product(r, p), r_prime);

        Vector3 { x, y, z }
    }

    #[inline]
    pub fn rotate(self, axis: Vector3, angle: Angle) -> Vector3 {
        let r = quaternion::get_rotation(angle, axis);
        self.apply_rotation(r)
    }

    #[inline]
    pub fn multiply_scalar(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    #[inline]
    pub fn normalize(self) -> Vector3 {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.multiply_scalar(1.0 / mag)
        } else {
            self
        }
    }

    pub fn rotate_xyz(self, other: Vector3) -> Vector3 {
        let Vector3 { x, y, z } = other;
        self.rotate((1.0, 0.0, 0.0).into(), Angle::from_radians(x))
            .rotate((0.0, 1.0, 0.0).into(), Angle::from_radians(y))
            .rotate((0.0, 0.0, 1.0).into(), Angle::from_radians(z))
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    #[inline]
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Vector3 { x, y, z }
    }
}

impl From<Vector3> for (f64, f64, f64) {
    fn from(Vector3 { x, y, z }: Vector3) -> (f64, f64, f64) {
        (x, y, z)
    }
}
