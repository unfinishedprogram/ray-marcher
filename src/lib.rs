#![feature(local_key_cell_methods)]
#![feature(async_closure)]

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
mod wgpu_context;
use angle::Angle;
use camera::Camera;
use entity::Entity;
use gpu_render::render_gpu;
use quaternion::{get_rotation, rotation_from_to};
use scene::SceneBuilder;
use signed_distance_field::{intersect, subtract, Primitive::*, SignedDistance};
use std::time::Duration;
use std::{borrow::Borrow, cell::RefCell};
use vector3::{Vector3, X, Y};

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement};
use wgpu_context::WgpuContext;

#[macro_use]
extern crate lazy_static;

use crate::scene_buffer::{SceneBufferBuilder, SceneEntity};

thread_local! {
    pub static G_CONTEXT: RefCell<Option<WgpuContext>> = RefCell::new(None);
}

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

    // render_gpu(soft, (192, 108)).await;

    let ctx = WgpuContext::new(&get_canvas()).await;
    G_CONTEXT.with_borrow_mut(move |value| {
        _ = value.insert(ctx);
    });

    gloo::timers::callback::Timeout::new(500, move || {
        G_CONTEXT.with_borrow_mut(|ctx| {
            let mut scene_buffer = SceneBufferBuilder::new();
            scene_buffer.push(SceneEntity::Sphere(1.0), false);
            scene_buffer.push(
                SceneEntity::Translate {
                    v: (3.0, 0.0, 0.0),
                    _padding: 0,
                    pointer: 0,
                },
                true,
            );

            scene_buffer.push(SceneEntity::Sphere(2.0), true);

            let scene_buffer = scene_buffer.build();
            _ = ctx.as_mut().unwrap().render(scene_buffer);
        })
    })
    .forget();
}
