pub mod objects;

use bytemuck::{Pod, Zeroable};
use objects::{Cuboid, Sphere};

#[derive(Clone, Default)]
pub struct SceneDescriptorBuilder {
    pub spheres: Vec<Sphere>,
    pub cuboids: Vec<Cuboid>,
}

impl SceneDescriptorBuilder {
    pub fn length_descriptor(&self) -> SceneLengthDescriptor {
        SceneLengthDescriptor {
            spheres: self.spheres.len() as u32,
            cuboids: self.cuboids.len() as u32,
            padding: [0; 2],
        }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SceneLengthDescriptor {
    spheres: u32,
    cuboids: u32,
    padding: [u32; 2],
}
