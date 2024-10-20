#![feature(async_closure)]

mod angle;
pub mod app;
mod camera;
mod frame_timer;
mod input;
mod light_buffers;
mod quaternion;
mod scene_buffer;
mod vector3;
mod wgpu_context;

use light_buffers::{Light, LightBufferBuilder, LightBuffers};
use scene_buffer::SceneBuffers;

use crate::scene_buffer::{SceneBufferBuilder, SceneEntity};

pub fn make_scene() -> (SceneBuffers, LightBuffers) {
    let mut scene_buffer = SceneBufferBuilder::new();

    let floor = scene_buffer.push(SceneEntity::Box {
        render: 0,
        dimensions: (10.0, 1.0, 10.0),
    });

    scene_buffer.push(SceneEntity::Translate {
        render: 1,
        pointer: floor,
        v: (0.0, -2.0, 0.0),
    });

    let b = scene_buffer.push(SceneEntity::Box {
        render: 0,
        dimensions: (1.0, 1.0, 1.0),
    });

    scene_buffer.push(SceneEntity::Translate {
        render: 1,
        pointer: b,
        v: (-2.0, 0.0, 0.0),
    });

    scene_buffer.push(SceneEntity::Sphere {
        render: 1,
        radius: 1.0,
    });

    let mut light_buffer = LightBufferBuilder::new();

    light_buffer.add(Light {
        position: (2.0, 3.0, 2.0),
        radius: 0.2,
        color: (0.2, 0.2, 1.0),
        enabled: 1,
    });

    light_buffer.add(Light {
        position: (-2.0, 3.0, 2.0),
        radius: 0.2,
        color: (1.0, 0.2, 0.2),
        enabled: 1,
    });

    (scene_buffer.build(), light_buffer.build())
}
