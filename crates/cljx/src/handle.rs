use ::std::{cell::{Ref, RefCell, RefMut}, cmp, fmt, hash::{Hash, Hasher}, rc::Rc};
use as_any::{AsAny, Downcast};
use crate::prelude::*;

mod write_handle;
mod buf_read_handle;

pub use write_handle::WriteHandle;
pub use buf_read_handle::BufReadHandle;

// pub trait IHandle: AsAny + fmt::Debug {}
pub trait IHandle: AsAny {}

#[derive(Clone)]
pub struct Handle(Rc<RefCell<dyn IHandle>>);


impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Handle {}


impl PartialOrd for Handle {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.as_ptr().cast::<()>()
            .partial_cmp(&other.0.as_ptr().cast::<()>())
    }
}

impl Ord for Handle {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.as_ptr().cast::<()>()
            .cmp(&other.0.as_ptr().cast::<()>())
    }
}


impl Hash for Handle {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        // TODO: a real hash impl, this is likely invalid and unsafe
        self.0.as_ptr().cast::<()>()
            .hash(hasher);
    }
}

impl Handle {
    pub fn borrow_ref(&self) -> Ref<'_, dyn IHandle> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, dyn IHandle> {
        self.0.borrow_mut()
    }

    pub fn downcast_ref<T>(&'_ self) -> Option<Ref<'_, T>>
    where T: IHandle + 'static
    {
        Ref::filter_map(self.borrow_ref(), Downcast::downcast_ref::<T>).ok()
    }

    pub fn downcast_mut<T>(&'_ self) -> Option<RefMut<'_, T>>
    where T: IHandle + 'static
    {
        RefMut::filter_map(self.borrow_mut(), Downcast::downcast_mut::<T>).ok()
    }
}

impl Handle {
    pub fn new<T: IHandle>(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let addr = self.0.as_ptr().cast::<()>();
        if let Some(ns) = self.downcast_ref::<RcNamespace>() {
            write!(f, "#handle[cljx.Namespace {:p} \"{}\"]", addr, ns.name_str())
        }
        else if let Some(_) = self.downcast_ref::<BufReadHandle>() {
            write!(f, "#handle[cljx.BufReadHandle {:p}]", addr)
        }
        else if let Some(_) = self.downcast_ref::<WriteHandle>() {
            write!(f, "#handle[cljx.WriteHandle {:p}]", addr)
        }
        else if let Some(func) = self.downcast_ref::<RcFunction>() {
            write!(f, "#handle[cljx.Function {:p} {}]", addr, func.name().unwrap_or("<unnamed>"))
        }
        else {
            write!(f, "#handle[{:p}]", addr)
        }
    }
}
