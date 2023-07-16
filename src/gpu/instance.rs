use wgpu::ShaderModule;

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
}
