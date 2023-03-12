use crate::{camera::Camera, primitives::sphere::Sphere};

mod camera;
mod combine;
mod primitives;
mod ray;
mod render;
mod scene;
mod vector3;
use combine::subtract;
use primitives::plane::Plane;
use render::render;
use scene::SceneBuilder;

fn main() {
    let scene = SceneBuilder::new(Camera::new(
        0.5,
        (0.0, 0.0, -10.0),
        (0.0, 0.0, 1.0),
        (16.0, 9.0),
    ))
    .add(Plane::new((0.0, 1.0, 0.0), (0.0, -1.0, 0.0)))
    .add(subtract(
        Sphere::new((0.0, 0.0, 0.0), 1.0),
        Sphere::new((-0.5, 0.0, -0.5), 1.0),
    ))
    .build();

    _ = render(&scene, (1920, 1080)).save("./test.png");
}
