use std::{cell::RefCell, io, rc::Rc};
use crate::prelude::*;

#[derive(Clone)]
pub struct WriteHandle(Rc<RefCell<dyn io::Write>>);

impl WriteHandle {
    pub fn new(writer: impl io::Write + 'static) -> Self {
        Self(Rc::new(RefCell::new(writer)))
    }
    /// Get the inner Rc<RefCell> directly to avoid nested borrows
    pub fn inner(&self) -> std::rc::Rc<RefCell<dyn io::Write>> {
        self.0.clone()
    }
}

impl IHandle for WriteHandle {}