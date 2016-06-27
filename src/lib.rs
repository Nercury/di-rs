//! <a href="https://github.com/Nercury/di-rs">
//! <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
//! </a>
//! <style>.sidebar { margin-top: 53px }</style>
//!

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate typedef;

mod deps;
mod collection;
mod scope;
mod inceptor;
mod constructed;

use std::result;
use std::error;

pub use constructed::MaybeMutexGuard;
pub use collection::Collection;
pub use scope::Scope;
pub use deps::Deps;

pub type Result<T> = result::Result<T, Box<error::Error>>;
