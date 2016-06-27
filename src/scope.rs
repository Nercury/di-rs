use std::any::Any;
use std::sync::LockResult;
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
        match self.obj {
            Instance::Isolated(obj) => obj,
            Instance::Shared(_arc) => unreachable!("can't explode shared"),
        }
    }

    pub fn lock<'a>(&'a mut self) -> LockResult<MaybeMutexGuard<'a, T>> {
        self.obj.lock()
    }

    pub fn get_instance(&self) -> &Instance<T> {
        &self.obj
    }
}
