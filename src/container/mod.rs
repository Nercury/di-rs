use std::any::Any;
use std::collections::HashMap;

use metafactory::{ AsFactoryExt, Factory };

pub struct Container {
    items: HashMap<String, Box<Any>>,
}

impl Container {
    pub fn new(items: HashMap<String, Box<Any>>) -> Container {
        Container {
            items: items,
        }
    }

    pub fn get<T:'static>(&self, id: &str) -> Option<Factory<T>> {
        match self.items.get(id) {
            Some(boxed) => boxed.as_factory_clone_of::<T>(),
            None => None,
        }
    }
}
