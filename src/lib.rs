#![feature(local_key_cell_methods)]
#![feature(async_closure)]

mod angle;
mod camera;
mod dimensions;
mod entity;
mod gpu;
mod input;
mod light;
mod light_buffers;
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
use dimensions::Dimensions;
use gloo::utils::window;
use input::Input;
use light_buffers::{Light, LightBufferBuilder};
use quaternion::multiply;
use vector3::Vector3;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu_context::WgpuContext;

use crate::{angle::Angle, quaternion::get_rotation, scene_buffer::SceneBufferBuilder, vector3::Y};

fn get_canvas() -> HtmlCanvasElement {
    JsCast::dyn_into(
        gloo::utils::document()
            .get_element_by_id("canvas")
            .expect("No canvas element"),
    )
    .expect("Not a valid canvas element")
}

pub fn make_scene() -> (SceneBufferBuilder, LightBufferBuilder) {
    let mut scene_buffer = SceneBufferBuilder::new();

    scene_buffer
        .r#box((10.0, 1.0, 10.0))
        .translate((0.0, -2.0, 0.0));

    scene_buffer.cylinder(1.0, 1.0);

    scene_buffer.sphere(1.0).translate((0.0, 2.0, 0.0));

    let mut light_buffer = LightBufferBuilder::new();

    light_buffer.add(Light {
        position: (2.0, 3.0, 2.0),
        radius: 0.05,
        color: (0.1, 0.1, 0.5),
        enabled: 1,
    });

    light_buffer.add(Light {
        position: (-2.0, 3.0, 2.0),
        radius: 0.05,
        color: (0.5, 0.1, 0.1),
        enabled: 1,
    });

    (scene_buffer, light_buffer)
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub async fn run() {
    let mut camera = Camera::new(0.5, (0.0, 0.0, -10.0), (0.0, 0.0, 0.0, 1.0), 0.001, 1000.0);

    let canvas = get_canvas();

    let mut yaw = 0.0;
    let mut pitch = 0.0;

    let mut input = Input::new(&get_canvas());

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let (scene, lights) = make_scene();
    let mut ctx = WgpuContext::new(
        canvas,
        &[
            (&Dimensions::new(32, 32), 0),
            (&scene, 1),
            (&lights, 2),
            (&camera, 3),
        ],
    )
    .await;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mouse = input.mouse_movement();

        yaw -= mouse.0;
        pitch -= mouse.1;

        pitch = pitch.clamp(-90.0, 90.0);
        yaw %= 360.0;

        let yaw_quat = get_rotation(Angle::from_degrees(yaw), Y);
        let pitch_quat = get_rotation(Angle::from_degrees(pitch), (0.0, 0.0, -1.0));

        camera.orientation = multiply(multiply(yaw_quat, (0.0, 0.0, 0.0, 1.0)), pitch_quat);

        camera.position.add_assign(
            input
                .keyboard_movement()
                .apply_rotation(get_rotation(Angle::from_degrees(yaw), Y)),
        );

        ctx.render(make_scene(), &camera).unwrap();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
