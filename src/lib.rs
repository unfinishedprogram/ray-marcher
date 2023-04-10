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

use std::{cell::RefCell, rc::Rc};

use camera::Camera;
use gloo::utils::window;
use quaternion::rotation_from_to;
use scene_buffer::SceneBuffers;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu_context::WgpuContext;

use crate::{
    angle::Angle,
    quaternion::get_rotation,
    scene_buffer::{SceneBufferBuilder, SceneEntity},
    vector3::Y,
};

fn get_canvas() -> HtmlCanvasElement {
    JsCast::dyn_into(
        gloo::utils::document()
            .get_element_by_id("canvas")
            .expect("No canvas element"),
    )
    .expect("Not a valid canvas element")
}

fn make_scene(angle: f32) -> SceneBuffers {
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
        q: get_rotation(Angle::from_degrees(angle), Y),
    });

    scene_buffer.push(SceneEntity::Sphere {
        render: 1,
        radius: 1.0,
    });

    scene_buffer.build()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
async fn run() {
    log::set_max_level(log::LevelFilter::Info);
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");

    let mut angle = 0.0;

    let mut camera = Camera::new(
        0.5,
        (0.0, 0.0, -10.0),
        get_rotation(Angle::from_degrees(0.0), (0.0, 1.0, 0.0)),
        0.001,
        1000.0,
    );

    let ctx = WgpuContext::new(&get_canvas()).await;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // Only do one full turn
        if angle > 360.0 {
            // Drop our handle to this closure so that it will get cleaned
            let _ = f.borrow_mut().take();
            return;
        }

        let rot = get_rotation(Angle::from_degrees(angle), (0.0, 1.0, 0.0));
        camera.orientation = rot;
        ctx.render(make_scene(0.0), &camera).unwrap();

        angle += 1.0;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
