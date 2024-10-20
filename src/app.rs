use crate::{
    angle::Angle,
    camera::Camera,
    frame_timer::FrameTimer,
    input::Input,
    make_scene,
    quaternion::get_rotation,
    vector3::{Vector3, Y},
    wgpu_context::WgpuContext,
};
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
            yaw: 0.0,
            pitch: 0.0,
            camera: Camera::new(0.5, (0.0, 0.0, -10.0), (0.0, 0.0, 0.0, 1.0), 0.001, 1000.0),
            input,
            frame_timer: FrameTimer::new(30),
        }
    }

    fn render_frame(&mut self) {
        // let mouse = input.mouse_movement();

        // yaw -= mouse.0;
        // pitch -= mouse.1;

        // pitch = pitch.clamp(-90.0, 90.0);
        // yaw %= 360.0;

        // let yaw_quat = get_rotation(Angle::from_degrees(yaw), Y);
        // let pitch_quat = get_rotation(Angle::from_degrees(pitch), (0.0, 0.0, -1.0));

        // // camera.orientation = multiply(multiply(yaw_quat, camera.orientation), pitch_quat);
        // camera.orientation = multiply(multiply(yaw_quat, (0.0, 0.0, 0.0, 1.0)), pitch_quat);

        self.camera.position.add_assign(
            self.input
                .camera_translation()
                .apply_rotation(get_rotation(Angle::from_degrees(self.yaw), Y)),
        );

        self.ctx.render(make_scene(), &self.camera).unwrap();
        println!("{:}", self.frame_timer.mark_frame());
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
