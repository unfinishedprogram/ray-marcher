use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, Buffer, BufferBinding, CommandBuffer, ComputePipeline,
    Extent3d, ImageCopyBuffer, ImageDataLayout, RenderPipeline, ShaderModule, ShaderStages,
    Texture, TextureViewDescriptor,
};

use crate::{
    camera::Camera,
    dimensions::Dimensions,
    gpu::{instance::WgpuInstance, Resource, ResourceGroup},
    light_buffers::LightBufferBuilder,
    scene_buffer::SceneBufferBuilder,
};

pub struct WgpuContext {
    pub instance: WgpuInstance,
    pub render_pipeline: RenderPipeline,
    pub compute_pipeline: ComputePipeline,
    pub dims: Dimensions,
    pub scene_resource_group: ResourceGroup,
    pub fragment_module: ShaderModule,
    pub vertex_module: ShaderModule,

    pub render_buffer: Buffer,
    pub render_texture: Texture,

    pub surface_bind_group: BindGroup,
    pub compute_bind_group: BindGroup,
}

impl<'a> WgpuContext {
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        resources: &'a [(&dyn Resource<'a>, u32)],
    ) -> Self {
        let mut instance = WgpuInstance::new(canvas).await;

        let scene_resource_group: ResourceGroup = ResourceGroup::new(&instance.device, resources);

        let vertex_module = instance.load_shader_module(wgpu::include_wgsl!("shaders/vert.wgsl"));
        let fragment_module =
            instance.load_shader_module(wgpu::include_wgsl!("shaders/draw_texture.wgsl"));
        let compute_module =
            instance.load_shader_module(wgpu::include_wgsl!("shaders/compute.wgsl"));

        let dims = Dimensions::new(
            instance.surface_config.width,
            instance.surface_config.height,
        );

        let render_buffer = instance.create_render_buffer();
        let render_texture = instance.create_render_texture();

        let surface_bind_group_layout =
            instance
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Surface BindGroup Layout"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            visibility: ShaderStages::FRAGMENT,
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            count: None,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                        },
                    ],
                });

        let render_pipeline = instance.create_surface_render_pipeline(
            &fragment_module,
            &vertex_module,
            &surface_bind_group_layout,
        );
        let compute_buffer_bg_layout =
            instance
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    }],
                });

        let compute_pipeline = instance.create_compute_pipeline(
            &compute_module,
            &[
                &scene_resource_group.bind_group_layout,
                &compute_buffer_bg_layout,
            ],
        );

        let texture_view = render_texture.create_view(&TextureViewDescriptor::default());
        let texture_sampler = instance.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let surface_bind_group = instance
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &surface_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    },
                ],
                label: Some("Surface BindGroup"),
            });

        let compute_bind_group = instance.device.create_bind_group(&BindGroupDescriptor {
            label: Some("buffer"),
            layout: &compute_buffer_bg_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(render_buffer.as_entire_buffer_binding()),
            }],
        });

        Self {
            instance,
            render_pipeline,
            compute_pipeline,
            vertex_module,
            fragment_module,
            scene_resource_group,
            surface_bind_group,
            compute_bind_group,
            render_buffer,
            render_texture,
            dims,
        }
    }

    pub fn render_to_buffer(
        &mut self,
        objects: &SceneBufferBuilder,
        lights: &LightBufferBuilder,
        camera: &Camera,
    ) {
        let scene_bind_group = self.scene_resource_group.bind_group_entries(
            &self.instance.device,
            &[(&self.dims, 0), (objects, 1), (lights, 2), (camera, 3)],
        );

        let mut encoder =
            self.instance
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &scene_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups(
                self.instance.surface_config.width / 16,
                self.instance.surface_config.height / 16,
                1,
            );
        }

        self.instance.queue.submit([encoder.finish()]);
    }

    pub fn copy_from_buffer(&self) -> CommandBuffer {
        let mut encoder =
            self.instance
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Copy Encoder"),
                });
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &self.render_buffer,
                layout: ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16 * self.instance.surface_config.width),
                    rows_per_image: Some(self.instance.surface_config.height),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &self.render_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: Default::default(),
            },
            Extent3d {
                width: self.instance.surface_config.width,
                height: self.instance.surface_config.height,
                depth_or_array_layers: 1,
            },
        );
        encoder.finish()
    }

    pub fn render_to_surface(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output_texture = self.instance.surface.get_current_texture()?;

        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.instance
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
            render_pass.set_bind_group(0, &self.surface_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
        self.copy_from_buffer();
        self.instance
            .queue
            .submit([self.copy_from_buffer(), encoder.finish()]);
        output_texture.present();
        Ok(())
    }
}
