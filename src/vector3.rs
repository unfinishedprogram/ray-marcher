use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn magnitude_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_sq().sqrt()
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        (
            other.y * self.z - other.z * self.y,
            other.z * self.x - other.x * self.z,
            other.x * self.y - other.y * self.x,
        )
            .into()
    }

    pub fn rotate(&self, axis: Vector3, angle: f32) -> Vector3 {
        let (sin_a, cos_a) = angle.sin_cos();
        let dot_product = self.dot(axis);
        let cross_product = self.cross(axis);
        axis.multiply_scalar(dot_product * (1.0 - cos_a))
            + *self
            + cross_product.multiply_scalar(sin_a)
    }

    pub fn multiply_scalar(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

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

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vector3 { x, y, z }
    }
}
