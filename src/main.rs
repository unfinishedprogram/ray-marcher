use pollster::FutureExt;
use ray_marcher::app::App;
use wasm_bindgen_futures::spawn_local;
use winit::{event_loop::EventLoop, window::Window};

// pub fn main() {
//     log::set_max_level(log::LevelFilter::Info);
//     std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
//     console_log::init().expect("could not initialize logger");
//     log::info!("Logger Initialized");

//     spawn_local(run());
// }

pub fn main() {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    #[allow(unused_mut)]
    let mut window_attributes = Window::default_attributes();
    // #[cfg(target_arch = "wasm32")]
    // {
    //     use wasm_bindgen::JsCast;
    //     let canvas = web_sys::window()
    //         .unwrap()
    //         .document()
    //         .unwrap()
    //         .get_element_by_id("canvas")
    //         .unwrap()
    //         .dyn_into::<web_sys::HtmlCanvasElement>()
    //         .unwrap();
    //     builder = builder.with_canvas(Some(canvas));
    // }

    #[allow(deprecated)]
    let window = event_loop.create_window(window_attributes).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();

        let mut app = App::create(&window).block_on();

        app.run(event_loop).block_on()
    }
    // #[cfg(target_arch = "wasm32")]
    // {
    //     std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //     console_log::init().expect("could not initialize logger");
    //     wasm_bindgen_futures::spawn_local(run(event_loop, window));
    // }
}
