use crate::{camera::Camera, primitives::sphere::Sphere};

mod angle;
mod camera;
mod combine;
mod light;
mod primitives;
pub mod quaternion;
mod ray;
mod render;
mod scene;
mod util;
mod vector3;
use angle::Angle;
use combine::subtract;
use primitives::plane::Plane;
use quaternion::get_rotation;
use render::render;
use scene::SceneBuilder;
use vector3::Vector3;

fn main() {
    let scene = SceneBuilder::new(Camera::new(
        Angle::from_degrees(60.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(20.0), (1.0, 0.0, 0.0).into()),
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
    .light((2.0, -3.0, 0.0), (0.5, 0.5, 4.0))
    .light((0.0, -4.9, 0.0), (0.5, 4.0, 0.5))
    .light((-2.0, -3.0, 0.0), (4.0, 0.5, 0.5))
    .build();

    _ = render(&scene, (1920, 1080)).save("./test.png");
}
