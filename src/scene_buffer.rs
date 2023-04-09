use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
const MAX_ENTITIES: usize = 4;
const ENTITY_SIZE: usize = size_of::<SceneEntity>();

use crate::{quaternion::Quaternion, vector3::Vec3};

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

// unsafe impl Pod for SceneEntity {}
unsafe impl Pod for SceneEntity {}
unsafe impl Zeroable for SceneEntity {}

unsafe impl Pod for SceneBuffers {}
unsafe impl Zeroable for SceneBuffers {}

pub struct SceneBufferBuilder {
    entities: [SceneEntity; MAX_ENTITIES],
    entities_length: usize,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct SceneBuffers {
    entities: [u8; MAX_ENTITIES * ENTITY_SIZE],
    entities_length: u32,
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

    pub fn build(self) -> SceneBuffers {
        SceneBuffers {
            entities: bytemuck::cast(self.entities),
            entities_length: self.entities_length as u32,
        }
    }
}
