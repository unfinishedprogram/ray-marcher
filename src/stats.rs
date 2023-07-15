use std::fmt::Display;

use gloo::utils::document;
use web_sys::Element;

#[derive(Clone, Copy)]
pub struct StatHandle {
    inner: usize,
}

pub trait StatValue: Sized + Display {}

pub struct StatTracker {
    element: Element,
    stats: Vec<String>,
    nodes: Vec<Element>,
}

impl StatTracker {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            stats: vec![],
            nodes: vec![],
        }
    }
    pub fn new_stat(&mut self, name: impl Into<String>) -> StatHandle {
        self.stats.push("".into());
        let row_elm = document()
            .create_element("tr")
            .expect("Failed to create table row");
        let name_elm = document()
            .create_element("td")
            .expect("Failed to create table entry");
        let value_elm = document()
            .create_element("td")
            .expect("Failed to create table entry");
        name_elm.set_text_content(Some(&name.into()));
        row_elm.append_child(&name_elm).unwrap();
        row_elm.append_child(&value_elm).unwrap();
        self.element.append_child(&row_elm).unwrap();
        self.nodes.push(value_elm);

        StatHandle {
            inner: self.stats.len() - 1,
        }
    }

    pub fn update(&self, handle: &StatHandle, value: impl Into<String>) {
        self.nodes[handle.inner].set_text_content(Some(&value.into()));
    }
}
