/*!

Has a `Container`, which contains compiled facotry list by name.

*/

use std::any::Any;
use std::collections::HashMap;

use metafactory::{ AsFactoryExt, Factory };

/// Contains the compiled factory list.
///
/// It is a convenience container, used for getting `Factory<T>` items
/// based on `T` type.
pub struct Container {
    items: HashMap<String, Box<Any>>,
}

impl Container {
    /// Create a new factory container.
    ///
    /// `Box<Any>` items should contain `Factory<T>` type.
    pub fn new(items: HashMap<String, Box<Any>>) -> Container {
        Container {
            items: items,
        }
    }

    /// Return a factory item if it can be downcasted to `Factory<T>` type.
    pub fn get<T:'static>(&self, id: &str) -> Option<Factory<T>> {
        match self.items.get(id) {
            Some(boxed) => boxed.as_factory_clone_of::<T>(),
            None => None,
        }
    }
}
