use wgpu::{
    util::DeviceExt, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Features,
    Limits, PipelineCompilationOptions, RenderPipeline, ShaderModule,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{camera::Camera, light_buffers::LightBuffers, scene_buffer::SceneBuffers};

pub struct WgpuContext<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub fragment_module: ShaderModule,
    pub vertex_module: ShaderModule,
    pub render_pipeline: RenderPipeline,
    pub size: (u32, u32),

    pub bind_group_layout: BindGroupLayout,
}

fn load_shaders(device: &wgpu::Device) -> (ShaderModule, ShaderModule) {
    let vertex_module = device.create_shader_module(wgpu::include_wgsl!("shaders/vert.wgsl"));
    let fragment_module = device.create_shader_module(wgpu::include_wgsl!("shaders/frag.wgsl"));

    (vertex_module, fragment_module)
}

impl<'a> WgpuContext<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        log::info!("Creating Wgpu Context");

        let (width, height) = (size.width, size.height);

        log::info!("Canvas size: {width}x{height}");

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        // Request adapter with high perf power preference
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to get requested adapter");

        log::info!("{:?}", adapter.get_downlevel_capabilities());

        log::info!("Backend: {:?}", adapter.get_info().backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to request device");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width,
            height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        surface.configure(&device, &surface_config);

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let (vertex_module, fragment_module) = load_shaders(&device);

        let render_pipeline = {
            let frag_targets = [Some(wgpu::ColorTargetState {
                format: surface_config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })];

            let fragment = Some(wgpu::FragmentState {
                module: &fragment_module,
                entry_point: "main",
                targets: &frag_targets,
                compilation_options: PipelineCompilationOptions::default(),
            });

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

            let desc = &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_module,
                    entry_point: "main",
                    buffers: &[],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            };

            device.create_render_pipeline(desc)
        };

        Self {
            render_pipeline,
            vertex_module,
            fragment_module,
            surface,
            device,
            queue,
            config: surface_config,
            size: (width, height),
            bind_group_layout,
        }
    }

    pub fn render(
        &self,
        scene: (SceneBuffers, LightBuffers),
        camera: &Camera,
    ) -> Result<(), wgpu::SurfaceError> {
        let (scene, lights) = scene;

        let output_texture = self.surface.get_current_texture()?;

        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let dimension_uniform = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Dimension Buffer"),
                // Padded to 16 bytes, uniforms must be.
                contents: bytemuck::bytes_of(&[self.size.0 as f32, self.size.1 as f32, 1.0, 1.0]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let scene_data = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene Buffer"),
                contents: bytemuck::bytes_of(&scene),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let light_data = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Light Buffer"),
                contents: bytemuck::bytes_of(&lights),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let camera_uniform = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::bytes_of(camera),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        {
            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Main Bind group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: dimension_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: scene_data.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: light_data.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: camera_uniform.as_entire_binding(),
                    },
                ],
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);

            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();
        Ok(())
    }

    pub fn resize(&mut self, _window: &Window, new_size: PhysicalSize<u32>) {
        // Reconfigure the surface with the new size
        self.config.width = new_size.width.max(1);
        self.config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.config);
    }
}
