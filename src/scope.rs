use std::any::Any;
use std::sync::Arc;
use std::sync::LockResult;
use std::mem;
use constructed::{Instance, AnyInstance, MaybeMutexGuard};

#[derive(Debug)]
pub struct Scope<T> {
    obj: Instance<T>,
    childs: Vec<Box<Any>>,
}

impl<T: Any> Scope<T> {
    pub fn from_any_instance(obj: AnyInstance, childs: Vec<Box<Any>>) -> Scope<T> {
        Scope {
            obj: obj.downcast(),
            childs: childs,
        }
    }

    pub fn explode(self) -> T {
        mem::drop(self.childs); // Childs contain a special "destructor" that
                                // will free up the arc when dropped.
                                // To make behaviour consistent, we are dropping childs before
                                // parent in all cases.
        match self.obj {
            Instance::Isolated(obj) => obj,
            Instance::Shared(arc) => {
                Arc::try_unwrap(arc)
                    .ok()
                    .expect("expected arc to be last remaining")
                    .into_inner()
                    .expect("expected to lock value before exploding")
            }
        }
    }

    pub fn lock<'a>(&'a mut self) -> LockResult<MaybeMutexGuard<'a, T>> {
        self.obj.lock()
    }

    pub fn get_instance(&self) -> &Instance<T> {
        &self.obj
    }
}
