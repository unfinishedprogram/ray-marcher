use bytemuck::{Pod, Zeroable};

const MAX_LIGHTS: usize = 8;

use crate::{gpu, vector3::Vec3};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Light {
    pub position: Vec3,
    pub radius: f32,
    pub color: Vec3,
    pub enabled: u32,
}

unsafe impl Pod for Light {}
unsafe impl Zeroable for Light {}

pub struct LightBufferBuilder {
    lights: [Light; MAX_LIGHTS],
    index: usize,
}
impl LightBufferBuilder {
    pub fn new() -> Self {
        Self {
            lights: [Default::default(); MAX_LIGHTS],
            index: 0,
        }
    }

    pub fn add(&mut self, light: Light) {
        self.lights[self.index] = light;
        self.index += 1;
    }
}

impl Default for LightBufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> gpu::Resource<'a> for LightBufferBuilder {
    fn buffer_init_descriptor(&'a self, _binding: u32) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::bytes_of(&self.lights),
            usage: wgpu::BufferUsages::UNIFORM,
        }
    }
}
