#![feature(local_key_cell_methods)]
#![feature(async_closure)]

mod angle;
mod camera;
mod entity;
mod input_handler;
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
use gloo::utils::{body, window};
use input_handler::InputHandler;
use quaternion::multiply;
use scene_buffer::SceneBuffers;
use vector3::Vector3;
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

    let mouse_pos = Rc::new(RefCell::new((0, 0)));
    let mut camera = Camera::new(0.5, (0.0, 0.0, -10.0), (0.0, 0.0, 0.0, 1.0), 0.001, 1000.0);

    let canvas = get_canvas();
    let m = mouse_pos.clone();

    let mut yaw = 0.0;
    let mut pitch = 0.0;

    let mouse_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        let mut mouse = m.borrow_mut();
        mouse.0 += event.movement_x();
        mouse.1 += event.movement_y();
    });

    let click_closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
        gloo::console::console_dbg!("REQUEST PTR LOCK");
        get_canvas().request_pointer_lock();
    });

    let ctx = WgpuContext::new(&canvas).await;

    let input_handler = InputHandler::new(&body());

    canvas
        .add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref())
        .unwrap();

    canvas
        .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
        .unwrap();

    mouse_closure.forget();
    click_closure.forget();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut mouse = mouse_pos.borrow_mut();

        yaw += (-mouse.0 as f32) / 10.0;
        pitch += (-mouse.1 as f32) / 10.0;
        pitch = pitch.clamp(-45.0, 45.0);
        yaw %= 360.0;

        // Reset mouse delta values since change has been handled
        (mouse.0, mouse.1) = (0, 0);

        let yaw_quat = get_rotation(Angle::from_degrees(yaw), Y);
        let pitch_quat = get_rotation(Angle::from_degrees(pitch), (0.0, 0.0, -1.0));

        // camera.orientation = multiply(multiply(yaw_quat, camera.orientation), pitch_quat);
        camera.orientation = multiply(multiply(yaw_quat, (0.0, 0.0, 0.0, 1.0)), pitch_quat);

        camera.position.add_assign(
            input_handler
                .get_movement(0.5)
                .apply_rotation(get_rotation(Angle::from_degrees(yaw), Y)),
        );

        ctx.render(make_scene(0.0), &camera).unwrap();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
