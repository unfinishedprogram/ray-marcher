use bytemuck::{Pod, Zeroable};

use crate::{
    angle::Angle,
    gpu,
    quaternion::Quaternion,
    vector3::{Vec3, Vector3},
};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
    pub orientation: Quaternion,
    pub clip_near: f32,
    pub clip_far: f32,
}

unsafe impl Pod for Camera {}
unsafe impl Zeroable for Camera {}

impl Camera {
    pub fn new(
        fov: f32,
        position: Vec3,
        orientation: Quaternion,
        clip_near: f32,
        clip_far: f32,
    ) -> Self {
        Camera {
            fov,
            position,
            orientation,
            clip_near,
            clip_far,
        }
    }

    pub fn get_ray_direction(&self, x: f32, y: f32) -> Vec3 {
        let y = -y + 0.5;
        let x = x - 0.5;

        (x, y, self.clip_near)
            .normalize()
            .apply_rotation(self.orientation)
    }
}

impl<'a> gpu::Resource<'a> for Camera {
    fn buffer_init_descriptor(&'a self, _binding: u32) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(self),
            usage: wgpu::BufferUsages::UNIFORM,
        }
    }
}
