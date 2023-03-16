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
use material::Material;
use quaternion::{get_rotation, rotation_from_to};
use render::render;
use scene::SceneBuilder;
use signed_distance_field::{subtract, union, Plane, Sphere, Torus, Transform};

fn main() {
    let basic_white = Material::Basic((1.0, 1.0, 1.0));

    let micky = union(
        union(
            Transform::translate(Sphere(0.5), (-0.6, 0.6, 0.0)),
            Transform::translate(Sphere(0.5), (0.6, 0.6, 0.0)),
        ),
        Sphere(0.7),
    );

    let floor = Entity::new(
        Transform::translate(
            subtract(
                Plane,
                Transform::rotate(
                    micky,
                    get_rotation(Angle::from_degrees(90.0), (1.0, 0.0, 0.0)),
                ),
            ),
            (0.0, -5.0, 0.0),
        ),
        basic_white.clone(),
    );

    let wall = Entity::new(
        Transform::new(
            Plane,
            (0.0, 0.0, 5.0),
            rotation_from_to((0.0, 1.0, 0.0), (0.0, 0.0, -1.0)),
        ),
        basic_white.clone(),
    );

    let sphere_1 = Entity::new(
        Transform::translate(Sphere(0.5), (-3.0, 0.0, 0.0)),
        basic_white.clone(),
    );

    let sphere_2 = Entity::new(
        Transform::translate(Sphere(0.5), (3.0, 0.0, 0.0)),
        basic_white.clone(),
    );

    let cutout = Entity::new(
        subtract(
            Transform::translate(Sphere(0.5), (0.0, 0.0, 0.0)),
            Transform::translate(Sphere(0.5), (-0.5, 0.0, -0.5)),
        ),
        basic_white.clone(),
    );

    let ring = Entity::new(
        Transform::new(
            Torus(0.5, 0.25),
            (0.0, -4.0, 0.0),
            get_rotation(Angle::from_degrees(0.0), (1.0, 0.0, 0.0)),
        ),
        basic_white,
    );

    let scene = SceneBuilder::new(Camera::new(
        Angle::from_degrees(60.0),
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
