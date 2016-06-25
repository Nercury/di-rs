/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!

*/
#![feature(specialization)]

mod deps;
pub mod extension;

use std::any::Any;
use std::fmt;
use std::slice;
use std::convert;
pub use deps::{ Deps, Features, Scope };

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

pub struct CollectionIter<'a, T: 'a> {
    inner: slice::Iter<'a, T>,
}

impl<'a, T: 'a> Iterator for CollectionIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }
}

impl<'a, T> IntoIterator for &'a Collection<T> {
    type IntoIter = CollectionIter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> CollectionIter<'a, T> {
        CollectionIter { inner: self.items.iter() }
    }
}

impl<T> Into<Vec<T>> for Collection<T> {
    fn into(self) -> Vec<T> {
        self.items
    }
}

pub struct Expect<T: Any> {
    response: Option<T>,
}

impl<T: Any> Expect<T> {
    pub fn load(deps: &Deps) -> Result<T, ()> {
        let expectation = Expect::<T> {
            response: None,
        };
        let maybe_fullfilled = deps.create_for(expectation).unwrap().explode();
        match maybe_fullfilled.response {
            Some(value) => Ok(value),
            None => Err(()),
        }
    }

    pub fn replace(&mut self, value: T) -> Result<(), ()> {
        if let Some(_) = self.response {
            return Err(());
        }

        self.response = Some(value);

        Ok(())
    }
}

pub fn load_from<T: Any>(deps: &Deps) -> Result<T, ()> {
    Expect::load(deps)
}
