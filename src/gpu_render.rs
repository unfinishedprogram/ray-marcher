use crate::{
    scene_buffer::{SceneBufferBuilder, SceneEntity},
    util::show_image,
    Vector3,
};
use image::RgbImage;
use log::info;
use wgpu::{util::DeviceExt, Device, Queue};

use crate::scene::Scene;

pub async fn get_compute_device(instance: &wgpu::Instance) -> (Device, Queue) {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to get adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    (device, queue)
}

pub async fn render_gpu(scene: Scene, (width, height): (usize, usize)) {
    log::warn!("Hello!");
    let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU);
    let (device, queue) = get_compute_device(&instance).await;
    let (width, height) = (width as u64, height as u64);
    let buffer_size = width * height * 4 * 4;

    let mut scene_buffer = SceneBufferBuilder::new();
    scene_buffer.push(SceneEntity::Sphere(1.0), true);
    // scene_buffer.push(
    //     SceneEntity::Translate {
    //         v: (2.0, 0.0, 0.0),
    //         _padding: 0,
    //         pointer: 0,
    //     },
    //     true,
    // );

    let scene_buffer = scene_buffer.build();

    info!("Device Info:\n {device:?}");

    let cs_module = device.create_shader_module(wgpu::include_wgsl!("shaders/depth.wgsl"));

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let contents: Vec<f32> = vec![0.0; buffer_size as usize];

    log::info!("{:?}", contents.len());

    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&contents),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
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
                resource: storage_buffer.as_entire_binding(),
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

    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, buffer_size);

    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);

    if let Some(Ok(())) = receiver.receive().await {
        let data = buffer_slice.get_mapped_range();
        log::info!("Done");
        let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        log::info!("{:?}", &result);
        let result = result
            .chunks_exact(4)
            .flat_map(|arr| {
                let (r, g, b) = (arr[0], arr[1], arr[2]);
                let (r, g, b) = (r, g, b).add((0.5, 0.5, 0.5)).rgb_u8();
                [r, g, b]
            })
            .collect();

        show_image(RgbImage::from_raw(width as u32, height as u32, result).unwrap());
        drop(data);
        staging_buffer.unmap();
    }
}
