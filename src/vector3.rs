use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    #[inline]
    pub fn magnitude_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vector3) -> f32 {
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

    #[inline]
    pub fn rotate(&self, axis: Vector3, angle: f32) -> Vector3 {
        let (sin_a, cos_a) = angle.sin_cos();
        let dot_product = self.dot(axis);
        let cross_product = self.cross(axis);
        axis.multiply_scalar(dot_product * (1.0 - cos_a))
            + *self
            + cross_product.multiply_scalar(sin_a)
    }

    #[inline]
    pub fn multiply_scalar(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    #[inline]
    pub fn normalize(self) -> Vector3 {
        let mag = self.magnitude();
        if mag == 0.0 {
            self
        } else {
            self.multiply_scalar(1.0 / mag)
        }
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

impl From<(f32, f32, f32)> for Vector3 {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vector3 { x, y, z }
    }
}
