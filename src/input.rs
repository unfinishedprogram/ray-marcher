pub mod input_tracker;
mod keyboard;
pub mod keyboard_provider;

use input_tracker::InputTracker;
use keyboard_provider::KeyboardProvider;

use crate::vector3::{Vec3, Vector3};

pub struct Input {
    pub keyboard: InputTracker,
    movement_speed: f32,
    sensitivity: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: InputTracker::default(),
            movement_speed: 0.05,
            sensitivity: 0.1,
        }
    }

    pub fn movement(&self) -> Vec3 {
        self.keyboard
            .movement()
            .multiply_scalar(self.movement_speed)
    }
}
