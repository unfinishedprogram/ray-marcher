use bytemuck::{Pod, Zeroable};
const MAX_ENTITIES: usize = 8;

use crate::{
    gpu,
    transform::Transform,
    vector3::{Vec3, Vector3},
};

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub enum SceneEntity {
    Empty,

    Sphere {
        transform: Transform,
        radius: f32,
    },

    Box {
        transform: Transform,
        dimensions: Vec3,
    },

    Cylinder {
        transform: Transform,
        radius: f32,
        height: f32,
    },
}

impl SceneEntity {
    pub fn sphere(radius: f32) -> Self {
        SceneEntity::Sphere {
            transform: Default::default(),
            radius,
        }
    }

    pub fn r#box(dimensions: Vec3) -> Self {
        SceneEntity::Box {
            transform: Default::default(),
            dimensions,
        }
    }

    pub fn cylinder(radius: f32, height: f32) -> Self {
        SceneEntity::Cylinder {
            transform: Default::default(),
            radius,
            height,
        }
    }

    pub fn translate(mut self, translation: Vec3) -> Self {
        match &mut self {
            SceneEntity::Empty => {}
            SceneEntity::Sphere { transform, .. }
            | SceneEntity::Box { transform, .. }
            | SceneEntity::Cylinder { transform, .. } => {
                transform.translation.add_assign(translation);
            }
        };
        self
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
