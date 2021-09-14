#![allow(clippy::type_complexity)]
//! <a href="https://github.com/Nercury/di-rs">
//! <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_darkblue_121621.png" alt="Fork me on GitHub">
//! </a>
//! <style>.sidebar { margin-top: 53px }</style>
//!

mod collection;
mod constructed;
mod deps;
mod inceptor;
mod scope;

use std::error;
use std::result;

pub use collection::Collection;
pub use constructed::MaybeMutexGuard;
pub use deps::Deps;
pub use scope::Scope;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
