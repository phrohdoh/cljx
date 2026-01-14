use std::{cell::{RefCell, RefMut}, io, rc::Rc};
use crate::IHandle;

#[derive(Clone)]
pub struct WriteHandle(Rc<RefCell<dyn io::Write>>);

impl WriteHandle {
    pub fn new(writer: impl io::Write + 'static) -> Self {
        Self(Rc::new(RefCell::new(writer)))
    }
    fn as_io_write_mut(&self) -> RefMut<'_, dyn io::Write + 'static > {
        self.0.borrow_mut()
    }
    /// Get the inner Rc<RefCell> directly to avoid nested borrows
    pub fn inner(&self) -> std::rc::Rc<RefCell<dyn io::Write>> {
        self.0.clone()
    }
}

impl IHandle for WriteHandle {}