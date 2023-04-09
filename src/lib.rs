#![feature(local_key_cell_methods)]
#![feature(async_closure)]

mod angle;
mod camera;
mod entity;
mod light;
mod material;
mod quaternion;
mod render;
mod scene;
mod scene_buffer;
mod signed_distance_field;
mod util;
mod vector3;
mod wgpu_context;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu_context::WgpuContext;

use crate::{
    angle::Angle,
    quaternion::get_rotation,
    scene_buffer::{SceneBufferBuilder, SceneEntity},
    vector3::{X, Y, Z},
};

fn get_canvas() -> HtmlCanvasElement {
    JsCast::dyn_into(
        gloo::utils::document()
            .get_element_by_id("canvas")
            .expect("No canvas element"),
    )
    .expect("Not a valid canvas element")
}

#[wasm_bindgen(start)]
async fn run() {
    log::set_max_level(log::LevelFilter::Info);
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");

    log::info!("Starting");

    // let soft = SceneBuilder::new(Camera::new(
    //     Angle::from_degrees(30.0),
    //     16.0 / 9.0,
    //     (0.0, 0.0, -10.0),
    //     get_rotation(Angle::from_degrees(25.0), X),
    //     (1.0, 200.0),
    // ))
    // .add(Entity::new(Plane.translate_y(-5.0)))
    // .add(Entity::new(
    //     Box((1.0, 1.0, 1.0).rotate(Y, Angle::from_degrees(0.0))).translate_y(-4.0),
    // ))
    // .light((-5.0, -2.0, 2.0), (25.0, 25.0, 25.0), 0.1)
    // .build();

    let mut ctx = WgpuContext::new(&get_canvas()).await;

    let mut scene_buffer = SceneBufferBuilder::new();

    scene_buffer.push(SceneEntity::Box {
        render: 0,
        dimensions: (100.0, 1.0, 100.0),
    });

    scene_buffer.push(SceneEntity::Translate {
        render: 1,
        pointer: 0,
        v: (0.0, -5.0, 0.0),
    });

    scene_buffer.push(SceneEntity::Box {
        render: 0,
        dimensions: (1.0, 1.0, 1.0),
    });

    scene_buffer.push(SceneEntity::Translate {
        render: 0,
        pointer: 2,
        v: (0.0, -2.0, 0.0),
    });

    scene_buffer.push(SceneEntity::Rotate {
        render: 1,
        pointer: 3,
        q: get_rotation(Angle::from_degrees(45.0), Y),
    });

    scene_buffer.push(SceneEntity::Sphere {
        render: 1,
        radius: 1.0,
    });

    let scene_buffer = scene_buffer.build();
    dbg!(scene_buffer);
    _ = ctx.render(scene_buffer);
}
