use pollster::FutureExt;
use ray_marcher::app::App;
use winit::{event_loop::EventLoop, window::Window};

pub fn main() {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    #[allow(unused_mut)]
    let mut window_attributes = Window::default_attributes();

    #[allow(deprecated)]
    let window = event_loop.create_window(window_attributes).unwrap();

    env_logger::init();

    let mut app = App::create(&window).block_on();

    app.run(event_loop).block_on();
}
