mod keyboard;
mod mouse;

use gloo::utils::body;
use keyboard::KeyboardHandler;
use mouse::MouseHandler;

use crate::vector3::Vector3;

pub struct Input {
    keyboard: KeyboardHandler,
    mouse: MouseHandler,

    movement_speed: f32,
    sensitivity: f32,
}

impl Input {
    pub fn new(element: &web_sys::Element) -> Self {
        Self {
            keyboard: KeyboardHandler::new(&body()),
            mouse: MouseHandler::new(element),
            movement_speed: 0.25,
            sensitivity: 0.1,
        }
    }
}

impl Input {
    pub fn mouse_movement(&mut self) -> (f32, f32) {
        let (x, y) = self.mouse.movement();
        (x as f32 * self.sensitivity, y as f32 * self.sensitivity)
    }

    pub fn keyboard_movement(&self) -> (f32, f32, f32) {
        self.keyboard
            .movement()
            .multiply_scalar(self.movement_speed)
    }
}
