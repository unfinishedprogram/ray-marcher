use wgpu::{Maintain, RenderPipeline, ShaderModule};

use crate::{
    camera::Camera,
    dimensions::Dimensions,
    gpu::{Resource, ResourceGroup},
    light_buffers::LightBufferBuilder,
    scene_buffer::SceneBufferBuilder,
};

pub struct WgpuContext {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub fragment_module: ShaderModule,
    pub vertex_module: ShaderModule,
    pub render_pipeline: RenderPipeline,
    pub dims: Dimensions,

    pub resource_group: ResourceGroup,
}

fn load_shaders(device: &wgpu::Device) -> (ShaderModule, ShaderModule) {
    let vertex_module = device.create_shader_module(wgpu::include_wgsl!("shaders/vert.wgsl"));
    let fragment_module = device.create_shader_module(wgpu::include_wgsl!("shaders/frag.wgsl"));

    (vertex_module, fragment_module)
}

impl<'a> WgpuContext {
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        resources: &'a [(&dyn Resource<'a>, u32)],
    ) -> Self {
        log::info!("Creating Wgpu Context");
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

        let resource_group: ResourceGroup = ResourceGroup::new(&device, resources);

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
            });

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&resource_group.bind_group_layout],
                push_constant_ranges: &[],
            });

            let desc = &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &vertex_module,
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

            device.create_render_pipeline(desc)
        };

        Self {
            render_pipeline,
            vertex_module,
            fragment_module,
            surface,
            device,
            queue,
            resource_group,
            config: surface_config,
            dims: Dimensions::new(width, height),
        }
    }

    pub fn render(
        &mut self,
        objects: &SceneBufferBuilder,
        lights: &LightBufferBuilder,
        camera: &Camera,
    ) -> Result<(), wgpu::SurfaceError> {
        let bind_group = self.resource_group.bind_group_entries(
            &self.device,
            &[(&self.dims, 0), (objects, 1), (lights, 2), (camera, 3)],
        );

        let output_texture = self.surface.get_current_texture()?;

        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);

            render_pass.draw(0..3, 0..1);
        }

        let submission = self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();

        Ok(())
    }
}
