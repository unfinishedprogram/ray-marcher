use std::collections::HashSet;

use winit::{event::ElementState, keyboard::Key};

#[derive(Default)]
pub struct KeyboardState {
    down: HashSet<Key>,
}

impl KeyboardState {
    pub fn on_keyboard_button(&mut self, key: Key, state: ElementState) {
        match state {
            ElementState::Pressed => self.down.insert(key),
            ElementState::Released => self.down.remove(&key),
        };
    }

    pub fn is_down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }
}
