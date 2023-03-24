use bytemuck::{Pod, Zeroable};

type Ptr = u32;
type Vec3 = (f32, f32, f32);

#[repr(C)]
#[derive(Clone, Copy)]
pub enum SceneEntity {
    Sphere(f32),
    Translate(Vec3, Ptr),
}

unsafe impl Pod for SceneEntity {}
unsafe impl Zeroable for SceneEntity {}

pub fn main() {
    let sphere = bytemuck::bytes_of(&SceneEntity::Sphere(2.0));
    let translate = bytemuck::bytes_of(&SceneEntity::Translate((1.0, 1.0, 1.0), 0));

    dbg!(sphere, sphere.len());

    dbg!(translate, translate.len());
}
