use ray_marcher::run;
use wasm_bindgen_futures::spawn_local;

pub fn main() {
    log::set_max_level(log::LevelFilter::Info);
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    log::info!("Logger Initialized");

    spawn_local(run());
}
