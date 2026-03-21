use std::{cell::RefCell, io, rc::Rc};
use crate::prelude::*;

#[derive(Clone)]
pub struct BufReadHandle(Rc<RefCell<dyn io::BufRead>>);

impl BufReadHandle {
    pub fn new(writer: impl io::BufRead + 'static) -> Self {
        Self(Rc::new(RefCell::new(writer)))
    }
    /// Get the inner Rc<RefCell> directly to avoid nested borrows
    pub fn inner(&self) -> std::rc::Rc<RefCell<dyn io::BufRead>> {
        self.0.clone()
    }
}

impl IHandle for BufReadHandle {}