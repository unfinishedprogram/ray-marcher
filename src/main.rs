use crate::{camera::Camera, primitives::sphere::Sphere};

mod angle;
mod camera;
mod combine;
mod light;
mod primitives;
mod ray;
mod render;
mod scene;
mod vector3;
use angle::Angle;
use combine::subtract;
use primitives::plane::Plane;
use render::render;
use scene::SceneBuilder;
use vector3::Vector3;

fn main() {
    let scene = SceneBuilder::new(Camera::new(
        Angle::from_degrees(90.0),
        16.0 / 9.0,
        (0.0, 0.0, -5.0),
        (0.0, 0.0, 1.0),
        (1.0, 200.0),
    ))
    .add(Plane::new((0.0, 1.0, 0.0), (0.0, -5.0, 0.0)))
    .add(Plane::new((0.0, 0.0, -1.0), (0.0, 0.0, 10.0)))
    .add(subtract(
        Sphere::new(Vector3::ZERO, 0.5),
        Sphere::new((-0.5, 0.0, -0.5), 0.5),
    ))
    .add(Sphere::new((-3.0, 0.0, 0.0), 0.5))
    .add(Sphere::new((3.0, 0.0, 0.0), 0.5))
    .light((10.0, 5.0, 0.0), 1.0)
    .build();

    _ = render(&scene, (1920, 1080)).save("./test.png");
}
