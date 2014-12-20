/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

#![feature(slicing_syntax)]
#![feature(default_type_params)]
#![feature(macro_rules)]

extern crate term;
extern crate typedef;
extern crate metafactory;

mod macros;
pub mod registry;
pub mod error_printer;
pub mod container;
pub mod factory_container;
