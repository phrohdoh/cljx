use std::{cell::{RefCell, RefMut}, io, rc::Rc};
use crate::prelude::*;

#[derive(Clone)]
pub struct BufReadHandle(Rc<RefCell<dyn io::BufRead>>);

impl BufReadHandle {
    pub fn new(writer: impl io::BufRead + 'static) -> Self {
        Self(Rc::new(RefCell::new(writer)))
    }
    fn as_io_read_mut(&self) -> RefMut<'_, dyn io::BufRead + 'static > {
        self.0.borrow_mut()
    }
    /// Get the inner Rc<RefCell> directly to avoid nested borrows
    pub fn inner(&self) -> std::rc::Rc<RefCell<dyn io::BufRead>> {
        self.0.clone()
    }
}

impl IHandle for BufReadHandle {}