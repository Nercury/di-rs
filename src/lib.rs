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
