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
    let micky = union(
        union(Sphere(0.5).translate_x(-0.6), Sphere(0.5).translate_x(0.6)).translate_y(0.6),
        Sphere(0.7),
    );

    let floor = Entity::new(
        subtract(
            Plane,
            micky.rotate(get_rotation(Angle::from_degrees(90.0), X)),
        )
        .translate((0.0, -5.0, 0.0)),
    );

    let ball_good = Sphere(1.0)
        .translate_x(5.0)
        .translate_y(0.5)
        .translate_z(1.0);

    let ball_bad = Sphere(1.0).translate((5.0, 0.5, 1.0));

    let wall = Entity::new(
        Plane
            .rotate(rotation_from_to((0.0, 1.0, 0.0), (0.0, 0.0, -1.0)))
            .translate((0.0, 0.0, 5.0)),
    );

    let sphere_1 = Entity::new(Sphere(0.5).translate((-3.0, 0.0, 0.0)));
    let sphere_2 = Entity::new(Sphere(0.5).translate((3.0, 0.0, 0.0)));
    let cutout = Entity::new(subtract(Sphere(0.5), Torus(0.5, 0.25)));

    let ring = Entity::new(
        Torus(0.5, 0.25)
            .rotate(get_rotation(Angle::from_degrees(0.0), (1.0, 0.0, 0.0)))
            .translate((0.0, -4.0, 0.0)),
    );

    let scene = SceneBuilder::new(Camera::new(
        Angle::from_degrees(30.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(20.0), (1.0, 0.0, 0.0)),
        (1.0, 200.0),
    ))
    .add(floor)
    .add(wall)
    .add(cutout)
    .add(sphere_1)
    .add(sphere_2)
    .add(ring)
    .light((2.0, -3.0, 0.0), (0.5, 0.5, 4.0))
    .light((0.0, -3.0, 0.0), (4.0, 0.5, 0.5))
    .light((-2.0, -3.0, 0.0), (0.5, 4.0, 0.5))
    .build();

    _ = render(&scene, (1920, 1080)).save("./test.png");
}
