use std::num::NonZeroU64;

use wgpu::{
    util::DeviceExt, Backends, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    Buffer, InstanceDescriptor, PipelineLayout, RenderPipelineDescriptor, ShaderModule,
};

use crate::scene_buffer::SceneBuffers;

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: (u32, u32),
}

impl WgpuContext {
    pub async fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        log::info!("Creating Wgpu Context");
        let (width, height) = (canvas.width(), canvas.height());
        log::info!("Canvas size: {width}x{height}");

        let instance = wgpu::Instance::new(InstanceDescriptor::default());
        let surface = instance.create_surface_from_canvas(canvas).unwrap();

        // Request adapter with high perf power preference
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to get requested adapter");

        log::info!("{:?}", adapter.get_downlevel_capabilities());

        log::info!("Backend: {:?}", adapter.get_info().backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // Since wgpu isn't well supported yet, we default to webgl2 as a fallback
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
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
        };

        surface.configure(&device, &surface_config);

        Self {
            surface,
            device,
            queue,
            config: surface_config,
            size: (width, height),
        }
    }

    fn load_shaders(&mut self) -> (ShaderModule, ShaderModule) {
        let vertex_module = self
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/vert.wgsl"));

        let fragment_module = self
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/frag.wgsl"));

        (vertex_module, fragment_module)
    }

    pub fn render(&mut self, scene: SceneBuffers) -> Result<(), wgpu::SurfaceError> {
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
                contents: bytemuck::bytes_of(&[self.size.0 as f32, self.size.1 as f32, 1.0, 1.0]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let scene_data = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene Uniform"),
                contents: bytemuck::bytes_of(&scene),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        {
            let bind_group_layout =
                self.device
                    .create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                        ],
                    });

            let render_pipeline = self.create_render_pipeline(&bind_group_layout);

            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Main Bind group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: dimension_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: scene_data.as_entire_binding(),
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
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);

            render_pass.draw(0..3, 0..1);
        }

        self.queue.on_submitted_work_done(|| {
            log::info!("DONE WORK");
        });

        self.queue.submit(std::iter::once(encoder.finish()));

        output_texture.present();
        Ok(())
    }

    fn create_render_pipeline(
        &mut self,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let (vert_module, frag_module) = self.load_shaders();

        let frag_targets = [Some(wgpu::ColorTargetState {
            format: self.config.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let fragment = Some(wgpu::FragmentState {
            module: &frag_module,
            entry_point: "main",
            targets: &frag_targets,
        });

        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[bind_group_layout],
                push_constant_ranges: &[],
            });

        let desc = &wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vert_module,
                entry_point: "main",
                buffers: &[],
            },
            fragment,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        };

        self.device.create_render_pipeline(desc)
    }
}
