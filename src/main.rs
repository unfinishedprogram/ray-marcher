use crate::camera::Camera;

mod angle;
mod camera;
mod light;
mod quaternion;
mod ray;
mod render;
mod scene;
mod signed_distance_field;
mod util;
mod vector3;
use angle::Angle;
use quaternion::get_rotation;
use render::render;
use scene::SceneBuilder;
use signed_distance_field::{subtract, Plane, Sphere};

fn main() {
    let scene = SceneBuilder::new(Camera::new(
        Angle::from_degrees(60.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(20.0), (1.0, 0.0, 0.0)),
        (1.0, 200.0),
    ))
    .add(Plane::new((0.0, 1.0, 0.0), (0.0, -5.0, 0.0)))
    .add(Plane::new((0.0, 0.0, -1.0), (0.0, 0.0, 10.0)))
    .add(subtract(
        Sphere::new((0.0, 0.0, 0.0), 0.5),
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
