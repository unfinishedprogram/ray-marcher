use wgpu::{RenderPipeline, ShaderModule};

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
    pub dims: Dimensions,
    pub resource_group: ResourceGroup,
    pub fragment_module: ShaderModule,
    pub vertex_module: ShaderModule,
}

impl<'a> WgpuContext {
    pub async fn new(
        canvas: web_sys::HtmlCanvasElement,
        resources: &'a [(&dyn Resource<'a>, u32)],
    ) -> Self {
        let mut instance = WgpuInstance::new(canvas).await;

        let resource_group: ResourceGroup = ResourceGroup::new(&instance.device, resources);

        let vertex_module = instance.load_shader_module(wgpu::include_wgsl!("shaders/vert.wgsl"));
        let fragment_module = instance.load_shader_module(wgpu::include_wgsl!("shaders/frag.wgsl"));

        let render_pipeline = instance.create_surface_render_pipeline(
            &fragment_module,
            &vertex_module,
            &resource_group.bind_group_layout,
        );

        let dims = Dimensions::new(
            instance.surface_config.width,
            instance.surface_config.height,
        );

        Self {
            instance,
            render_pipeline,
            vertex_module,
            fragment_module,
            resource_group,
            dims,
        }
    }

    pub fn render(
        &mut self,
        objects: &SceneBufferBuilder,
        lights: &LightBufferBuilder,
        camera: &Camera,
    ) -> Result<(), wgpu::SurfaceError> {
        let bind_group = self.resource_group.bind_group_entries(
            &self.instance.device,
            &[(&self.dims, 0), (objects, 1), (lights, 2), (camera, 3)],
        );

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
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.instance
            .queue
            .submit(std::iter::once(encoder.finish()));

        output_texture.present();

        Ok(())
    }
}
