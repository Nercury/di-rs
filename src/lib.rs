/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

#![feature(default_type_params)]

extern crate typedef;
extern crate metafactory;

pub mod registry;
pub mod getter;

pub struct Container;

impl Container {
    pub fn new() -> Container {
        Container
    }
}
