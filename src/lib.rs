/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!

*/

mod deps;
pub mod extension;

use std::ops::Deref;
use std::any::Any;
pub use deps::{ Deps, Scope, Parent };

pub trait WithAll<T> {
    fn with_all<A: Deref<Target=Deps>>(self, deps: A) -> Scope<T>;
}

impl<T: Any> WithAll<T> for T {
    fn with_all<A: Deref<Target=Deps>>(self, deps: A) -> Scope<T> {
        deps.create_deps(self)
    }
}
