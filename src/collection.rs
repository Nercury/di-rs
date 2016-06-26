use std::fmt;
use std::convert;
use std::slice;
use std::vec;

pub struct Collection<T> {
    items: Vec<T>,
}

impl<T> fmt::Debug for Collection<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}

impl<T> Collection<T> {
    pub fn new() -> Collection<T> {
        Collection {
            items: Vec::new()
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }
}

impl<T> convert::AsRef<[T]> for Collection<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type IntoIter = slice::Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Collection<T> {
    type IntoIter = slice::IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.items.iter_mut()
    }
}

impl<T> IntoIterator for Collection<T> {
    type IntoIter = vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> vec::IntoIter<T> {
        self.items.into_iter()
    }
}

impl<T> Into<Vec<T>> for Collection<T> {
    fn into(self) -> Vec<T> {
        self.items
    }
}
