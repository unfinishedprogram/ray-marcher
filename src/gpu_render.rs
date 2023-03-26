use crate::{
    scene_buffer::{SceneBufferBuilder, SceneEntity},
    util::show_image,
    Vector3,
};

use image::RgbImage;
use log::info;
use wasm_bindgen_futures::spawn_local;
use wgpu::{util::DeviceExt, Device, Queue};

use crate::scene::Scene;

pub async fn get_compute_device(instance: &wgpu::Instance) -> (Device, Queue) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("Failed to get adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Main Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    (device, queue)
}

pub async fn render_gpu(scene: Scene, (width, height): (usize, usize)) {
    log::info!("Fetching Device");

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let (device, queue) = get_compute_device(&instance).await;
    let (width, height) = (width as u64, height as u64);
    let buffer_size = width * height * 4;

    let view_buffer_size = width * height * 4;

    let mut scene_buffer = SceneBufferBuilder::new();
    scene_buffer.push(SceneEntity::Sphere(1.0), false);
    scene_buffer.push(
        SceneEntity::Translate {
            v: (3.0, 0.0, 0.0),
            _padding: 0,
            pointer: 0,
        },
        true,
    );

    scene_buffer.push(SceneEntity::Sphere(2.0), true);

    let scene_buffer = scene_buffer.build();

    info!("Device Info:\n {device:?}");

    let cs_module = device.create_shader_module(wgpu::include_wgsl!("shaders/depth.wgsl"));

    let view_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: view_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let view_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Storage Buffer"),
        size: view_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let dimension_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Dimension Buffer"),
        contents: bytemuck::cast_slice(&[width as u32, height as u32]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let scene_data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Scene Uniform"),
        contents: bytemuck::bytes_of(&scene_buffer),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: view_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: dimension_uniform.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: scene_data.as_entire_binding(),
            },
        ],
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        pass.set_pipeline(&compute_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.insert_debug_marker("Compute View Rays");
        pass.dispatch_workgroups(width as u32, height as u32, 1);
    }

    encoder.copy_buffer_to_buffer(&view_buffer, 0, &view_staging_buffer, 0, view_buffer_size);

    queue.submit(Some(encoder.finish()));
    let buffer_slice = view_staging_buffer.slice(..);

    // buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // if let Some(Ok(())) = receiver.receive().await {
    //     let data = buffer_slice.get_mapped_range();
    //     log::info!("Done");
    //     let result: Vec<u8> = bytemuck::cast_slice(&data).to_vec();

    //     let result = result
    //         .chunks_exact(4)
    //         .flat_map(|arr| {
    //             let (r, g, b) = (arr[0], arr[1], arr[2]);
    //             [r, g, b]
    //         })
    //         .collect();

    //     show_image(RgbImage::from_raw(width as u32, height as u32, result).unwrap());
    //     drop(data);
    //     view_staging_buffer.unmap();
    // }
}
