use std::any::Any;
use std::collections::HashMap;
use metafactory::factory::Factory;

pub struct Container {
    items: HashMap<String, Box<Any>>,
}

impl Container {
    pub fn new(items: HashMap<String, Box<Any>>) -> Container {
        Container {
            items: items,
        }
    }
}
