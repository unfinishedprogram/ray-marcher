use bytemuck::{Pod, Zeroable};
const MAX_ENTITIES: usize = 8;

use crate::{gpu, quaternion::Quaternion, vector3::Vec3};

type Ptr = u32;

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub enum SceneEntity {
    Empty,
    Sphere {
        render: u32,
        radius: f32,
    },

    Translate {
        render: u32,
        pointer: Ptr,
        v: Vec3,
    },

    Box {
        render: u32,
        dimensions: Vec3,
    },

    Rotate {
        render: u32,
        pointer: Ptr,
        q: Quaternion,
    },
}

unsafe impl Pod for SceneEntity {}
unsafe impl Zeroable for SceneEntity {}

pub struct SceneBufferBuilder {
    entities: [SceneEntity; MAX_ENTITIES],
    entities_length: usize,
}

impl SceneBufferBuilder {
    pub fn new() -> Self {
        Self {
            entities: [SceneEntity::Empty; MAX_ENTITIES],
            entities_length: 0,
        }
    }

    pub fn push(&mut self, entity: SceneEntity) -> u32 {
        let index = self.entities_length;
        self.entities[index] = entity;
        self.entities_length += 1;
        index as u32
    }
}

impl Default for SceneBufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> gpu::Resource<'a> for SceneBufferBuilder {
    fn buffer_init_descriptor(&'a self, _binding: u32) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some("Scene Buffer"),
            contents: bytemuck::bytes_of(&self.entities),
            usage: wgpu::BufferUsages::UNIFORM,
        }
    }
}
