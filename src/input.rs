pub mod keyboard_state;

use glam::{vec3, Vec3};
use keyboard_state::KeyboardState;
use winit::keyboard::Key;

pub struct Input {
    pub keyboard: KeyboardState,
    movement_speed: f32,
    sensitivity: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardState::default(),
            movement_speed: 0.05,
            sensitivity: 0.1,
        }
    }

    pub fn camera_translation(&self) -> Vec3 {
        let positive = vec3(
            self.keyboard.is_down(Key::Character("d".into())) as u32 as f32,
            self.keyboard.is_down(Key::Character("e".into())) as u32 as f32,
            self.keyboard.is_down(Key::Character("w".into())) as u32 as f32,
        );

        let negative = vec3(
            self.keyboard.is_down(Key::Character("a".into())) as u32 as f32,
            self.keyboard.is_down(Key::Character("q".into())) as u32 as f32,
            self.keyboard.is_down(Key::Character("s".into())) as u32 as f32,
        );

        (positive - negative) * self.movement_speed
    }
}
