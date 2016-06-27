use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, Arc, LockResult, PoisonError, MutexGuard};

enum MaybeMutexGuardValue<'a, T: 'a> {
    Guard(MutexGuard<'a, T>),
    Ref(&'a mut T),
}

pub struct MaybeMutexGuard<'a, T: 'a> {
    inner: MaybeMutexGuardValue<'a, T>,
}

#[derive(Debug)]
pub enum Instance<T> {
    Isolated(T),
    Shared(Arc<Mutex<T>>),
}

impl<T> Instance<T> {
    pub fn lock<'a>(&'a mut self) -> LockResult<MaybeMutexGuard<'a, T>> {
        match *self {
            Instance::Isolated(ref mut val) => {
                Ok(MaybeMutexGuard { inner: MaybeMutexGuardValue::Ref(val) })
            }
            Instance::Shared(ref mut val) => {
                match val.lock() {
                    Ok(guard) => Ok(MaybeMutexGuard { inner: MaybeMutexGuardValue::Guard(guard) }),
                    Err(e) => {
                        Err(PoisonError::new(MaybeMutexGuard {
                            inner: MaybeMutexGuardValue::Guard(e.into_inner()),
                        }))
                    }
                }
            }
        }
    }
}

impl<'a, T> Deref for MaybeMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.inner {
            MaybeMutexGuardValue::Guard(ref val) => val,
            MaybeMutexGuardValue::Ref(ref val) => val,
        }
    }
}

impl<'a, T> DerefMut for MaybeMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.inner {
            MaybeMutexGuardValue::Guard(ref mut val) => val,
            MaybeMutexGuardValue::Ref(ref mut val) => val,
        }
    }
}

#[derive(Debug)]
pub enum AnyInstance {
    Isolated(Box<Any>),
    Shared(Box<Any>),
}

impl AnyInstance {
    pub fn downcast<T: Any>(self) -> Instance<T> {
        match self {
            AnyInstance::Isolated(parent) => {
                Instance::Isolated(*parent.downcast()
                    .expect("expected AnyInstance::Isolated to downcast into type"))
            }
            AnyInstance::Shared(parent) => {
                Instance::Shared(*parent.downcast()
                    .expect("expected AnyInstance::Shared to downcast into type"))
            }
        }
    }
}

pub struct Constructed {
    pub children: Vec<Box<Any>>,
}

pub struct ConstructedShared {
    pub children: Vec<Box<Any>>,
}
