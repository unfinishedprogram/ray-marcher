use wgpu::{
    BindGroupLayout, Buffer, BufferUsages, ComputePipeline, ComputePipelineDescriptor,
    RenderPipeline, ShaderModule, Texture, TextureDescriptor,
};

pub struct WgpuInstance {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl WgpuInstance {
    pub async fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let (width, height) = (canvas.width(), canvas.height());
        log::info!("Canvas size: {width}x{height}");

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface_from_canvas(canvas).unwrap();

        // Request adapter with high perf power preference
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .expect("Failed to get requested adapter");

        log::info!("{:?}", adapter.get_downlevel_capabilities());
        log::info!("Backend: {:?}", adapter.get_info().backend);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

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

        WgpuInstance {
            surface,
            device,
            queue,
            surface_config,
        }
    }

    pub fn load_shader_module(&mut self, desc: wgpu::ShaderModuleDescriptor) -> ShaderModule {
        self.device.create_shader_module(desc)
    }

    // Entry point for both fragment and vertex shader must be "main"
    pub fn create_surface_render_pipeline(
        &self,
        fragment_module: &ShaderModule,
        vertex_module: &ShaderModule,
        bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let frag_targets = [Some(wgpu::ColorTargetState {
            format: self.surface_config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let fragment = Some(wgpu::FragmentState {
            module: fragment_module,
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
                module: vertex_module,
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

    pub fn create_render_buffer(&self) -> Buffer {
        // Enough to store each color as 4 f32
        // This allows for HDR and handling colors in fragment shader

        let size = self.surface_config.width as u64 * self.surface_config.height as u64 * 4 * 4;
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Render Buffer"),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
            size,
        })
    }

    pub fn create_render_texture(&self) -> Texture {
        let size = wgpu::Extent3d {
            width: self.surface_config.width,
            height: self.surface_config.height,
            depth_or_array_layers: 1,
        };

        self.device.create_texture(&TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            label: Some("Render Texture"),
        })
    }

    pub fn create_compute_pipeline(
        &self,
        compute_module: &ShaderModule,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> ComputePipeline {
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        let desc = ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&layout),
            module: compute_module,
            entry_point: "cs_main",
        };

        self.device.create_compute_pipeline(&desc)
    }
}
