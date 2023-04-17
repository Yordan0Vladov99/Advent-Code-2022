use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct SystemFile {
    pub name: String,
    pub size: u32,
    pub children: Vec<Rc<RefCell<SystemFile>>>,
    pub parent: Option<Rc<RefCell<SystemFile>>>,
}
