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
use std::result;
use std::error;
use std::vec;
pub use deps::{ Deps, Scope };

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

pub struct Expect<T: Any> {
    response: Option<T>,
}

impl<T: Any> Expect<T> {
    pub fn load(deps: &Deps) -> Result<T> {
        let expectation = Expect::<T> {
            response: None,
        };
        let maybe_fullfilled = try!(deps.create(expectation)).explode();
        match maybe_fullfilled.response {
            Some(value) => Ok(value),
            None => Err(Box::new(Error::ExpectedDependencyNotFound)),
        }
    }

    pub fn replace(&mut self, value: T) -> Result<()> {
        if let Some(_) = self.response {
            return Err(Box::new(Error::ExpectedDependencyWasAlreadyFullfilled));
        }

        self.response = Some(value);

        Ok(())
    }
}

pub fn load_from<T: Any>(deps: &Deps) -> Result<T> {
    Expect::load(deps)
}

#[derive(Debug)]
pub enum Error {
    ExpectedDependencyNotFound,
    ExpectedDependencyWasAlreadyFullfilled,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ExpectedDependencyNotFound => "Expected dependency not found".fmt(f),
            Error::ExpectedDependencyWasAlreadyFullfilled => "Expected dependency was already fullfilled".fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::ExpectedDependencyNotFound => "expected dependency not found",
            Error::ExpectedDependencyWasAlreadyFullfilled => "expected dependency was already fullfilled",
        }
    }
}

pub type Result<T> = result::Result<T, Box<error::Error>>;
