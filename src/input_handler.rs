use std::{cell::RefCell, collections::HashSet, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::vector3::{Vec3, Vector3};

pub struct InputHandler {
    keys_pressed: Rc<RefCell<HashSet<String>>>,
}

fn bool_tuple_to_vec((a, b, c): (bool, bool, bool)) -> Vec3 {
    let x = if a { 1.0 } else { 0.0 };
    let y = if b { 1.0 } else { 0.0 };
    let z = if c { 1.0 } else { 0.0 };

    (x, y, z)
}

impl InputHandler {
    pub fn new(element: &web_sys::Element) -> Self {
        let keys_pressed = Rc::new(RefCell::new(HashSet::new()));

        let k = keys_pressed.clone();
        let key_down = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut keys = k.borrow_mut();
            keys.insert(event.key());
        });

        let k = keys_pressed.clone();
        let key_up = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut keys = k.borrow_mut();
            keys.remove(&event.key());
        });

        element
            .add_event_listener_with_callback("keydown", key_down.as_ref().unchecked_ref())
            .unwrap();

        element
            .add_event_listener_with_callback("keyup", key_up.as_ref().unchecked_ref())
            .unwrap();

        key_up.forget();
        key_down.forget();

        Self { keys_pressed }
    }

    pub fn is_down(&self, key: &str) -> bool {
        self.keys_pressed.borrow().contains(key)
    }

    // Returns a vector representing the players directed movement in 3D
    pub fn get_movement(&self, speed: f32) -> Vec3 {
        let positive = bool_tuple_to_vec((self.is_down("d"), self.is_down("q"), self.is_down("s")));
        let negative = bool_tuple_to_vec((self.is_down("a"), self.is_down("e"), self.is_down("w")))
            .multiply_scalar(-1.0);

        Vector3::add(positive, negative)
            .normalize()
            .multiply_scalar(speed)
    }
}
