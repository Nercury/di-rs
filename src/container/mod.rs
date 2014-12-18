use std::any::Any;
use std::collections::HashMap;

pub struct Container {
    _items: HashMap<String, Box<Any>>,
}

impl Container {
    pub fn new(items: HashMap<String, Box<Any>>) -> Container {
        Container {
            _items: items,
        }
    }
}
