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
pub use deps::{ Deps, Features, Scope, Parent };

pub struct Expect<T: Any> {
    response: Option<T>,
}

impl<T: Any> Expect<T> {
    pub fn load(deps: &Deps) -> Result<T, ()> {
        let expectation = Expect::<T> {
            response: None,
        };
        let maybe_fullfilled = deps.create_for(expectation).explode();
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
