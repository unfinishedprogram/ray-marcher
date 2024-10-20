use std::{cell::RefCell, collections::HashSet, rc::Rc};

use wasm_bindgen::prelude::*;

pub struct KeyboardHandler {
    keys_pressed: Rc<RefCell<HashSet<String>>>,
}

impl KeyboardHandler {
    pub fn new(element: &web_sys::HtmlElement) -> Self {
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
}
