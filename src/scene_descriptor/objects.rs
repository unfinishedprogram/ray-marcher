use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sphere {
    pub transform: Mat4,
    pub radius: f32,
    _padding: [u32; 3],
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Cuboid {
    pub transform: Mat4,
    pub dimensions: Vec3,
    _padding: u32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self {
            transform: Mat4::IDENTITY,
            radius,
            _padding: [0; 3],
        }
    }

    pub fn translate(&self, v: Vec3) -> Self {
        Self {
            transform: Mat4::from_translation(v) * self.transform,
            ..*self
        }
    }
}

impl Cuboid {
    pub fn new(dimensions: Vec3) -> Self {
        Self {
            transform: Mat4::IDENTITY,
            dimensions,
            _padding: 0,
        }
    }

    pub fn translate(&self, v: Vec3) -> Self {
        Self {
            transform: Mat4::from_translation(v) * self.transform,
            ..*self
        }
    }

    pub fn rotate(&self, v: Vec3) -> Self {
        Self {
            transform: Mat4::from_rotation_x(v.x)
                * Mat4::from_rotation_y(v.y)
                * Mat4::from_rotation_z(v.z)
                * self.transform,
            ..*self
        }
    }
}
