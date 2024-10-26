use bytemuck::{Pod, Zeroable};

const MAX_LIGHTS: usize = 8;
const LIGHT_SIZE: usize = std::mem::size_of::<Light>();

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Light {
    pub position: glam::Vec3,
    pub radius: f32,
    pub color: glam::Vec3,
    pub enabled: u32,
}

unsafe impl Pod for Light {}
unsafe impl Zeroable for Light {}

unsafe impl Pod for LightBuffers {}
unsafe impl Zeroable for LightBuffers {}

#[derive(Clone, Copy, Debug)]
#[repr(C, align(16))]
pub struct LightBuffers {
    lights: [u8; MAX_LIGHTS * LIGHT_SIZE],
    lights_length: u32,
}
pub struct LightBufferBuilder {
    lights: [Light; MAX_LIGHTS],
    index: usize,
}
impl LightBufferBuilder {
    pub fn new() -> Self {
        Self {
            lights: [Default::default(); MAX_LIGHTS],
            index: 0,
        }
    }

    pub fn add(&mut self, light: Light) {
        self.lights[self.index] = light;
        self.index += 1;
    }

    pub fn build(self) -> LightBuffers {
        LightBuffers {
            lights: bytemuck::cast(self.lights),
            lights_length: self.lights.len() as u32,
        }
    }
}

impl Default for LightBufferBuilder {
    fn default() -> Self {
        Self::new()
    }
}
