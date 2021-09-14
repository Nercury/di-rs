use std::convert;
use std::fmt;
use std::slice;
use std::vec;

pub struct Collection<T> {
    items: Vec<T>,
}

impl<T> fmt::Debug for Collection<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.items.iter()).finish()
    }
}

impl<T> Collection<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Collection<T> {
        Collection { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.into_iter()
    }
}

impl<T> convert::AsRef<[T]> for Collection<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.items.iter()
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> vec::IntoIter<T> {
        self.items.into_iter()
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    fn from(collection: Collection<T>) -> Self {
        collection.items
    }
}
