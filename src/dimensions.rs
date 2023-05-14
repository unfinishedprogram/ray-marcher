use crate::gpu;

pub struct Dimensions {
    buffer: [f32; 4],
}
impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: [width as f32, height as f32, 0.0, 0.0],
        }
    }
}

impl<'a> gpu::Resource<'a> for Dimensions {
    fn buffer_init_descriptor(&'a self, _binding: u32) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: Some("Dimension Buffer"),
            contents: bytemuck::bytes_of(&self.buffer),
            usage: wgpu::BufferUsages::UNIFORM,
        }
    }
}
