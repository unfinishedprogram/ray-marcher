#![feature(async_closure)]

pub mod app;
mod camera;
mod frame_timer;
mod input;
mod light_buffers;
mod scene_buffer;
mod scene_descriptor;
mod wgpu_context;

use glam::vec3;
use light_buffers::{Light, LightBufferBuilder, LightBuffers};
use scene_descriptor::{
    objects::{Cuboid, Sphere},
    SceneDescriptorBuilder,
};

pub fn make_scene() -> (SceneDescriptorBuilder, LightBuffers) {
    let mut scene_buffer = SceneDescriptorBuilder::default();

    scene_buffer
        .cuboids
        .push(Cuboid::new(vec3(1.0, 1.0, 1.0)).translate(vec3(2.0, 0.0, 0.0)));

    scene_buffer
        .cuboids
        .push(Cuboid::new(vec3(10.0, 1.0, 10.0)).translate(vec3(0.0, 2.0, 0.0)));

    scene_buffer
        .spheres
        .push(Sphere::new(1.0).translate(vec3(0.0, 0.0, 0.0)));

    let mut light_buffer = LightBufferBuilder::new();

    light_buffer.add(Light {
        position: vec3(2.0, 3.0, 2.0),
        radius: 0.2,
        color: vec3(0.2, 0.2, 1.0),
        enabled: 1,
    });

    light_buffer.add(Light {
        position: vec3(-2.0, 3.0, 2.0),
        radius: 0.2,
        color: vec3(1.0, 0.2, 0.2),
        enabled: 1,
    });

    (scene_buffer, light_buffer.build())
}
