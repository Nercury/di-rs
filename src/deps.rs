use std::any::{ Any, TypeId };
use std::ops::{ Deref, DerefMut };
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Deps {
    /// List of functions that constructs all childs for a type
    /// and returns value wrapped in Any that must live as long as the parent type.
    type_child_constructors: HashMap<
        TypeId,
        Vec<Box<
            Fn(&Deps, &Any) -> Option<Box<Any>>
        >>
    >,
}

impl Deps {
    pub fn new() -> Deps {
        Deps {
            type_child_constructors: HashMap::new()
        }
    }

    /// Create dependencies for specified `obj` and return a wrapper `Scope` object.
    ///
    /// The wrapper `Scope` keeps ownership of all children together with parent object.
    pub fn create_deps<P: Any>(&self, obj: P) -> Scope<P> {
        match self.type_child_constructors.get(&TypeId::of::<P>()) {
            // if there are type child constructors
            Some(list) => {
                // run each child constructor and receive list of objects that will be kept inside
                // the parent scope.
                let deps: Vec<_> = list.iter()
                    .filter_map(|any_constructor| any_constructor(&self, &obj))
                    .collect();

                Scope { obj: obj, childs: deps }
            },
            // if there are no type childs, wrap the type in scope anyways with empty child list.
            None => Scope { obj: obj, childs: vec![] },
        }
    }

    /// Register child constructor that will be invoked when the parent `P` type is
    /// created.
    pub fn register_child_constructor<P: Any>(
        &mut self,
        any_constructor: Box<Fn(&Deps, &Any) -> Option<Box<Any>>>
    ) {
        match self.type_child_constructors.entry(TypeId::of::<P>()) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(any_constructor);
            },
            Entry::Vacant(e) => {
                e.insert(vec![any_constructor]);
            },
        };
    }

}

#[derive(Debug)]
pub struct Scope<T> {
    pub obj: T,
    childs: Vec<Box<Any>>,
}

impl<T> Deref for Scope<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.obj
    }
}

impl<T> DerefMut for Scope<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.obj
    }
}
