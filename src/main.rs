use crate::camera::Camera;

mod angle;
mod camera;
mod entity;
mod light;
mod material;
mod quaternion;
mod ray;
mod render;
mod scene;
mod signed_distance_field;
mod util;
mod vector3;
use angle::Angle;
use entity::Entity;
use quaternion::{get_rotation, rotation_from_to};
use render::render;
use scene::SceneBuilder;
use signed_distance_field::{subtract, union, Primitive::*, SignedDistance};
use vector3::{X, Y, Z};
fn main() {
    let apples = SceneBuilder::new(Camera::new(
        Angle::from_degrees(30.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(0.0), X),
        (1.0, 200.0),
    ))
    .add(Entity::new(
        subtract(Sphere(0.5), Torus(0.5, 0.25)).repeat(5.0),
    ))
    .light((0.0, 0.0, 0.0), (25.0, 25.0, 25.0))
    .build();

    let lights = SceneBuilder::new(Camera::new(
        Angle::from_degrees(30.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(0.0), X),
        (1.0, 200.0),
    ))
    .add(Entity::new(Plane.translate((0.0, -5.0, 0.0))))
    .add(Entity::new(
        Plane
            .rotate(rotation_from_to((0.0, 1.0, 0.0), (0.0, 0.0, -1.0)))
            .translate((0.0, 0.0, 5.0)),
    ))
    .add(Entity::new(
        Box((1.0, 1.0, 1.0))
            .rotate(get_rotation(Angle::from_degrees(45.0), Y))
            .translate((0.0, -2.0, 2.0)),
    ))
    .light((2.0, 0.0, 0.0), (0.5, 0.5, 4.0))
    .light((0.0, 0.0, 0.0), (4.0, 0.5, 0.5))
    .light((-2.0, 0.0, 0.0), (0.5, 4.0, 0.5))
    .build();

    // let sphere_1 = Entity::new(Sphere(0.5).translate((-3.0, 0.0, 0.0)));
    // let sphere_2 = Entity::new(Sphere(0.5).translate((3.0, 0.0, 0.0)));
    // let cutout = Entity::new(subtract(Sphere(0.5), Torus(0.5, 0.25)));

    // let scene = SceneBuilder::new(Camera::new(
    //     Angle::from_degrees(30.0),
    //     16.0 / 9.0,
    //     (0.0, 0.0, -10.0),
    //     get_rotation(Angle::from_degrees(0.0), X),
    //     (1.0, 200.0),
    // ))
    // .add(floor)
    // .add(wall)
    // .add(cutout)
    // .add(sphere_1)
    // .add(sphere_2)
    // .add(ring)

    // .build();

    _ = render(&lights, (1920, 1080)).save("./lights.png");
    _ = render(&apples, (1920, 1080)).save("./apples.png");
}
