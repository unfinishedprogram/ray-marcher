use std::time::Instant;

use crate::{
    camera::Camera, frame_timer::FrameTimer, input::Input, make_scene, wgpu_context::WgpuContext,
};
use glam::{quat, vec3, Quat};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    keyboard::{self, NamedKey},
    window::Window,
};

pub struct App<'a> {
    window: &'a Window,
    ctx: WgpuContext<'a>,
    yaw: f32,
    pitch: f32,
    camera: Camera,
    input: Input,
    frame_timer: FrameTimer,
}

impl<'a> App<'a> {
    pub async fn create(window: &'a Window) -> Self {
        let ctx = WgpuContext::new(window).await;
        let input = Input::new();

        Self {
            window,
            ctx,
            yaw: 0.1,
            pitch: 0.0,
            camera: Camera::new(
                0.5,
                vec3(0.0, 0.0, -10.0),
                quat(0.0, 0.0, 0.0, 1.0),
                0.001,
                1000.0,
            ),
            input,
            frame_timer: FrameTimer::new(30),
        }
    }

    fn render_frame(&mut self) {
        let start = Instant::now();
        let rotation_input = self.input.camera_rotation();

        // Accumulate yaw and pitch values
        self.yaw -= rotation_input.x;
        self.pitch -= rotation_input.y;

        // Clamp pitch to avoid gimbal lock
        self.pitch = self.pitch.clamp(-90.0, 90.0);
        self.yaw %= 360.0;

        let yaw_quat = Quat::from_rotation_y(self.yaw.to_radians());
        let pitch_quat = Quat::from_rotation_x(self.pitch.to_radians());

        // Apply yaw first, then pitch
        self.camera.orientation = pitch_quat * yaw_quat;

        // Move the camera
        let translation = self.input.camera_translation();
        // Orient the translation vector by the camera's orientation

        let translation = self.camera.orientation.inverse() * translation;
        self.camera.position += translation;

        self.ctx.render(make_scene(), &self.camera).unwrap();
        println!("{:}", self.frame_timer.mark_frame());
        let end = Instant::now();
        let elapsed = end - start;
        println!("Frame time: {:?}", elapsed);
    }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ctx.resize(self.window, new_size);
    }

    pub async fn run(&mut self, event_loop: winit::event_loop::EventLoop<()>) {
        event_loop.run_app(self).expect("Failure in event loop");
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => self.resize(new_size),
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.render_frame();
                self.window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if matches!(event.logical_key, keyboard::Key::Named(NamedKey::Escape)) {
                    event_loop.exit()
                }
                self.input
                    .keyboard
                    .on_keyboard_button(event.logical_key, event.state);
            }
            _ => {}
        }
    }
}
