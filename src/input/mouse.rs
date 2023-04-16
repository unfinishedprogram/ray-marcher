use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};

use crate::get_canvas;

pub struct MouseHandler {
    // In pixels
    movement: Rc<RefCell<(i32, i32)>>,
}

impl MouseHandler {
    pub fn new(element: &web_sys::Element) -> Self {
        let movement = Rc::new(RefCell::new((0, 0)));
        let m = movement.clone();
        let mouse_move = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut mouse = m.borrow_mut();
            mouse.0 += event.movement_x();
            mouse.1 += event.movement_y();
        });

        let click = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
            gloo::console::console_dbg!("REQUEST PTR LOCK");
            get_canvas().request_pointer_lock();
        });

        element
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())
            .unwrap();

        element
            .add_event_listener_with_callback("click", click.as_ref().unchecked_ref())
            .unwrap();

        mouse_move.forget();
        click.forget();

        Self { movement }
    }

    // Consumes movement,
    // This is so that mouse movements are always handled even in cases where frames are skipped
    pub fn movement(&mut self) -> (i32, i32) {
        let mut m = self.movement.borrow_mut();
        let res = (m.0, m.1);
        m.0 = 0;
        m.1 = 0;
        res
    }
}
