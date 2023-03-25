mod angle;
mod camera;
mod entity;
mod gpu_render;
mod light;
mod material;
mod quaternion;
mod render;
mod scene;
mod scene_buffer;
mod signed_distance_field;
mod util;
mod vector3;
use angle::Angle;
use camera::Camera;
use entity::Entity;
use gpu_render::render_gpu;
use quaternion::{get_rotation, rotation_from_to};
use scene::SceneBuilder;
use signed_distance_field::{intersect, subtract, Primitive::*, SignedDistance};
use vector3::{Vector3, X, Y};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() {
    log::set_max_level(log::LevelFilter::Info);
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");

    log::info!("Starting");

    let soft = SceneBuilder::new(Camera::new(
        Angle::from_degrees(30.0),
        16.0 / 9.0,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(25.0), X),
        (1.0, 200.0),
    ))
    .add(Entity::new(Plane.translate_y(-5.0)))
    .add(Entity::new(
        Box((1.0, 1.0, 1.0).rotate(Y, Angle::from_degrees(0.0))).translate_y(-4.0),
    ))
    .light((-5.0, -2.0, 2.0), (25.0, 25.0, 25.0), 0.1)
    .build();

    render_gpu(soft, (192, 108)).await;
}
