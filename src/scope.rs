use std::any::Any;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Scope<T> {
    pub obj: T,
    childs: Vec<Box<Any>>,
}

impl<T> Scope<T> {
    pub fn new(obj: T, childs: Vec<Box<Any>>) -> Scope<T> {
        Scope {
            obj: obj,
            childs: childs,
        }
    }

    pub fn explode(self) -> T {
        self.obj
    }
}

impl<T> Deref for Scope<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.obj
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj
    }
}
