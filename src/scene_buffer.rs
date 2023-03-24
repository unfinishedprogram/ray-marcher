use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
const MAX_ENTITIES: usize = 64;
const ENTITY_SIZE: usize = size_of::<SceneEntity>();

use crate::vector3::Vec3;

type Ptr = u32;

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub enum SceneEntity {
    Sphere(f32),
    Translate {
        v: Vec3,
        _padding: u32,
        pointer: Ptr,
    },
}

// unsafe impl Pod for SceneEntity {}
unsafe impl Pod for SceneEntity {}
unsafe impl Zeroable for SceneEntity {}

unsafe impl Pod for SceneBuffers {}
unsafe impl Zeroable for SceneBuffers {}

pub struct SceneBufferBuilder {
    entities: [SceneEntity; MAX_ENTITIES],
    render_queue: [u32; MAX_ENTITIES],

    entities_length: usize,
    render_queue_length: usize,
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct SceneBuffers {
    entities: [u8; MAX_ENTITIES * ENTITY_SIZE],
    render_queue: [u8; MAX_ENTITIES * 4],

    entities_length: u32,
    render_queue_length: u32,
}

impl SceneBufferBuilder {
    pub fn new() -> Self {
        Self {
            entities: [SceneEntity::Sphere(0.0); MAX_ENTITIES],
            render_queue: [0; MAX_ENTITIES],
            entities_length: 0,
            render_queue_length: 0,
        }
    }

    pub fn push(&mut self, entity: SceneEntity, render: bool) -> u32 {
        let index = self.entities_length;

        if render {
            self.render_queue[self.render_queue_length] = index as u32;
            self.render_queue_length += 1;
        }

        self.entities[index] = entity;
        self.entities_length += 1;
        index as u32
    }

    pub fn build(self) -> SceneBuffers {
        SceneBuffers {
            entities: bytemuck::cast(self.entities),
            render_queue: bytemuck::cast(self.render_queue),

            entities_length: self.render_queue_length as u32,
            render_queue_length: self.render_queue_length as u32,
        }
    }
}
