use wgpu::{
    util::DeviceExt, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer,
    Device,
};

pub trait Resource<'a> {
    fn bind_group_layout_entry(&self, binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn buffer_init_descriptor(&'a self, binding: u32) -> wgpu::util::BufferInitDescriptor<'a>;

    fn create_buffer<'b>(&'a self, device: &'b Device, binding: u32) -> wgpu::Buffer {
        let descriptor = self.buffer_init_descriptor(binding);
        device.create_buffer_init(&descriptor)
    }
}

pub struct ResourceGroup {
    // resources: &'a [(&'a dyn Resource<'a>, u32)],
    bind_group_buffers: Vec<Buffer>,
    pub bind_group_layout: BindGroupLayout,
}

impl<'a> ResourceGroup {
    pub fn new(device: &Device, resources: &'a [(&'a dyn Resource<'a>, u32)]) -> Self {
        let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = resources
            .iter()
            .map(|&(res, binding)| res.bind_group_layout_entry(binding))
            .collect();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries,
        });

        let bind_group_buffers: Vec<Buffer> = resources
            .iter()
            .map(|&(res, binding)| res.create_buffer(device, binding))
            .collect();

        Self {
            bind_group_layout,
            bind_group_buffers,
        }
    }

    pub fn bind_group_entries(
        &mut self,
        device: &Device,
        resources: &'a [(&'a dyn Resource<'a>, u32)],
    ) -> BindGroup {
        self.bind_group_buffers = resources
            .iter()
            .map(|&(res, binding)| res.create_buffer(device, binding))
            .collect();

        let bind_group_entries: Vec<BindGroupEntry> = resources
            .iter()
            .enumerate()
            .map(|(index, &(_res, binding))| BindGroupEntry {
                binding,
                resource: self.bind_group_buffers[index].as_entire_binding(),
            })
            .collect();

        device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &bind_group_entries,
        })
    }
}
