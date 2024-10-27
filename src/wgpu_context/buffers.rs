use wgpu::{util::DeviceExt, Device};

use crate::{
    camera::Camera, light_buffers::LightBuffers, scene_descriptor::SceneDescriptorBuilder,
};

pub struct GPUBuffers {
    pub dimension_uniform: wgpu::Buffer,
    pub scene_data: wgpu::Buffer,
    pub cuboids: wgpu::Buffer,
    pub spheres: wgpu::Buffer,
    pub light_data: wgpu::Buffer,
    pub camera_uniform: wgpu::Buffer,
}

impl GPUBuffers {
    pub fn update_buffers(
        &self,
        queue: &wgpu::Queue,
        dimensions: (u32, u32),
        scene: SceneDescriptorBuilder,
        lights: LightBuffers,
        camera: Camera,
    ) {
        queue.write_buffer(
            &self.dimension_uniform,
            0,
            bytemuck::bytes_of(&[dimensions.0 as f32, dimensions.1 as f32, 1.0, 1.0]),
        );
        queue.write_buffer(
            &self.scene_data,
            0,
            bytemuck::bytes_of(&scene.length_descriptor()),
        );
        queue.write_buffer(&self.cuboids, 0, bytemuck::cast_slice(&scene.cuboids));
        queue.write_buffer(&self.spheres, 0, bytemuck::cast_slice(&scene.spheres));
        queue.write_buffer(&self.light_data, 0, bytemuck::bytes_of(&lights));
        queue.write_buffer(&self.camera_uniform, 0, bytemuck::bytes_of(&camera));
    }

    pub fn create(
        device: &Device,
        dimensions: (u32, u32),
        scene: SceneDescriptorBuilder,
        lights: LightBuffers,
        camera: Camera,
    ) -> Self {
        let dimension_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Dimension Buffer"),
            // Padded to 16 bytes, uniforms must be.
            contents: bytemuck::bytes_of(&[dimensions.0 as f32, dimensions.1 as f32, 1.0, 1.0]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let scene_data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Buffer"),
            contents: bytemuck::bytes_of(&scene.length_descriptor()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let light_data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::bytes_of(&lights),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&camera),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cuboids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cuboid Buffer"),
            contents: bytemuck::cast_slice(&scene.cuboids),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let spheres = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Buffer"),
            contents: bytemuck::cast_slice(&scene.spheres),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            dimension_uniform,
            scene_data,
            light_data,
            camera_uniform,
            cuboids,
            spheres,
        }
    }
}
