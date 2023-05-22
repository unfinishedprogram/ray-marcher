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

    Cylinder {
        render: u32,
        radius: f32,
        height: f32,
    },

    Subtract {
        render: u32,
        pointer_a: u32,
        pointer_b: u32,
    },
}

impl SceneEntity {
    pub fn hide(&mut self) {
        match self {
            SceneEntity::Empty => {}
            SceneEntity::Sphere { render, .. }
            | SceneEntity::Translate { render, .. }
            | SceneEntity::Box { render, .. }
            | SceneEntity::Cylinder { render, .. }
            | SceneEntity::Subtract { render, .. }
            | SceneEntity::Rotate { render, .. } => *render = 0,
        }
    }
}

unsafe impl Pod for SceneEntity {}
unsafe impl Zeroable for SceneEntity {}

pub struct SceneBufferBuilder {
    entities: [SceneEntity; MAX_ENTITIES],
    pub entities_length: usize,
}

impl SceneBufferBuilder {
    pub fn new() -> Self {
        Self {
            entities: [SceneEntity::Empty; MAX_ENTITIES],
            entities_length: 0,
        }
    }

    pub fn push(&mut self, entity: SceneEntity) -> &mut Self {
        let index = self.entities_length;
        self.entities[index] = entity;
        self.entities_length += 1;
        self
    }

    pub fn translate(&mut self, v: Vec3) -> &mut Self {
        let index = self.entities_length - 1;
        self.entities[index].hide();

        self.push(SceneEntity::Translate {
            render: 1,
            pointer: index as u32,
            v,
        })
    }

    pub fn r#box(&mut self, dimensions: Vec3) -> &mut Self {
        self.push(SceneEntity::Box {
            render: 1,
            dimensions,
        })
    }

    pub fn sphere(&mut self, radius: f32) -> &mut Self {
        self.push(SceneEntity::Sphere { render: 1, radius })
    }

    pub fn cylinder(&mut self, height: f32, radius: f32) -> &mut Self {
        self.push(SceneEntity::Cylinder {
            render: 1,
            radius,
            height,
        })
    }

    pub fn subtract(&mut self, pointer_a: u32, pointer_b: u32) -> &mut Self {
        self.entities[pointer_a as usize].hide();
        self.entities[pointer_b as usize].hide();

        self.push(SceneEntity::Subtract {
            render: 1,
            pointer_a,
            pointer_b,
        })
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
