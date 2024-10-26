use bytemuck::{Pod, Zeroable};
use glam::{Quat, Vec3};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
    pub orientation: Quat,
    pub clip_near: f32,
    pub clip_far: f32,
}

unsafe impl Pod for Camera {}
unsafe impl Zeroable for Camera {}

impl Camera {
    pub fn new(fov: f32, position: Vec3, orientation: Quat, clip_near: f32, clip_far: f32) -> Self {
        Camera {
            fov,
            position,
            orientation,
            clip_near,
            clip_far,
        }
    }
}
