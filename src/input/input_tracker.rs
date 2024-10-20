use std::collections::HashMap;

use winit::{event::ElementState, keyboard::Key};

use super::keyboard_provider::KeyboardProvider;

#[derive(Default)]
pub struct InputTracker {
    key_states: HashMap<Key, ButtonState>,
}

impl InputTracker {
    pub fn on_keyboard_button(&mut self, key: Key, state: ElementState) {
        let key_state = self.key_states.entry(key).or_default();
        key_state.apply(state);
    }
}

impl KeyboardProvider for InputTracker {
    fn is_down(&self, key: Key) -> bool {
        self.key_states.get(&key).map_or(false, |state| state.down)
    }
}

#[derive(Default, Clone, Debug)]
pub struct ButtonState {
    pub pressed: bool,
    pub released: bool,
    pub down: bool,
}

impl ButtonState {
    fn update(&mut self) {
        self.pressed = false;
        self.released = false;
    }

    fn apply(&mut self, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.pressed = true;
                self.down = true;
            }
            ElementState::Released => {
                self.released = true;
                self.down = false;
            }
        }
    }
}
