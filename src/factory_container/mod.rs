use std::any::{ Any, AnyMutRefExt };
use std::boxed::BoxAny;
use metafactory::factory::{ Factory, Getter };

/// Proxy for configuring factory list without caring about the type used.
///
/// This is intended for internal use as injectable container of same-type
/// factories. Used for implementing `one_of` di configuration.
///
/// ```
/// # extern crate metafactory;
/// # extern crate di;
/// use std::any::Any;
/// use metafactory::{ metafactory, argless_as_factory, AsFactoryExt };
/// use di::factory_container::FactoryContainer;
///
/// fn main() {
///     // Let's say we know that we will have bunch of `bool` factories.
///     // In that case we create a new factory container for them:
///     let mut container = FactoryContainer::new::<bool>();
///
///     // Once we actually have our factories, we can inject them into container
///     // without dealing with types. Of course, we should make sure that types
///     // actually match before doing that in the code that is using this
///     // implementation.
///     container.push_items(vec![
///         argless_as_factory(|| true),
///         argless_as_factory(true),
///         argless_as_factory(|| 4i == 8),
///     ]);
///
///     // Once we are ready to use the factory, we can call `new_factory` to
///     // convert all dynamic stuff to statically constructed call hierarchy:
///     let anyed_bool_array_factory = container.new_factory();
///
///     // Of course, that returns it anyed (`Box<Any>`), but we can easily get un-anyed version
///     // by downcasting to `Factory<Vec<bool>>` or using a convenience extension
///     // method for that:
///     let bool_array_factory = anyed_bool_array_factory
///         .as_factory_of::<Vec<bool>>().unwrap();
///
///     // Calling it should produce expected boolean vector:
///     assert_eq!(bool_array_factory.take(), vec![true, true, false]);
///
///     // Of course, factory container itself should be usable as argument
///     // for other factories:
///     let metafactory_all_true = metafactory(|values: Vec<bool>| {
///         values.iter().fold(true, |a, &i| a & i)
///     });
///
///     // We can pass it when constructing a factory for this lambda metafactory:
///     let factory_all_true = metafactory_all_true.new(vec![
///         box bool_array_factory.clone() as Box<Any>
///     ])
///         .ok().unwrap() // check for errors here
///         .as_factory_of::<bool>().unwrap() // same story with downcasting
///     ;
///
///     assert_eq!(factory_all_true.take(), false); // not all values are true
/// }
/// ```
pub struct FactoryContainer<'a> {
    any_getter: Box<Any>,
    do_push_items: |&mut Box<Any>, Vec<Box<Any>>|:'a -> (),
    do_new_factory: |&mut Box<Any>|:'a -> Box<Any>, // Don't worry, it's like Javascript ;)
}

impl<'a> FactoryContainer<'a> {
    /// Create new factory container instance for specified type.
    pub fn new<T: 'static>() -> FactoryContainer<'a> {
        FactoryContainer {
            any_getter: box FactoryVecGetter::<T>::new(),
            do_push_items: |any_getter, items| {
                let getter: &mut FactoryVecGetter<T> = any_getter
                    .downcast_mut::<FactoryVecGetter<T>>().unwrap();

                let len = items.len();
                let items_iter = items.into_iter()
                    .map(|i| *i.downcast::<Factory<T>>().ok().unwrap());

                getter.factories.reserve_exact(len);
                getter.factories.extend(items_iter);
            },
            do_new_factory: |any_getter| {
                let getter: &mut FactoryVecGetter<T> = any_getter
                    .downcast_mut::<FactoryVecGetter<T>>().unwrap();

                box Factory::<Vec<T>>::new(getter.boxed_clone())
            }
        }
    }

    /// Push factory items into container.
    ///
    /// Note that all items should already match container type:
    /// if container was created for `int`, all pushed factories should
    /// produce int. Otherwise this method will panic your app.
    pub fn push_items(&mut self, items: Vec<Box<Any>>) {
        (self.do_push_items)(&mut self.any_getter, items);
    }

    /// Produces factory usable as argument for other factories.
    ///
    /// If inner factories make `int` values, this method will make factory
    /// that makes `Vec<int>` values.
    pub fn new_factory(&mut self) -> Box<Any> {
        (self.do_new_factory)(&mut self.any_getter)
    }
}

struct FactoryVecGetter<T: 'static> {
    factories: Vec<Factory<T>>,
}

impl<T> Clone for FactoryVecGetter<T> {
    fn clone(&self) -> FactoryVecGetter<T> {
        FactoryVecGetter::<T> {
            factories: self.factories.clone()
        }
    }
}

impl<T> FactoryVecGetter<T> {
    pub fn new() -> FactoryVecGetter<T> {
        FactoryVecGetter::<T> {
            factories: Vec::with_capacity(0)
        }
    }
}

impl<T> Getter<Vec<T>> for FactoryVecGetter<T> {
    fn take(&self) -> Vec<T> {
        // Reserve exact result size.
        let mut items = Vec::<T>::with_capacity(self.factories.len());

        // Construct results from factory results.
        items.extend(
            self.factories.iter()
                .map(|f| f.take())
        );

        // I love Rust.
        items
    }

    fn boxed_clone(&self) -> Box<Getter<Vec<T>> + 'static> {
        box self.clone()
    }
}

#[cfg(test)]
mod test {
    use metafactory::{ argless_as_factory, metafactory, AsFactoryExt };
    use super::{ FactoryContainer };

    #[test]
    fn should_be_usable_as_vec_of_types() {
        let mut container = FactoryContainer::new::<int>();
        container.push_items(vec![
            argless_as_factory(5i),
            argless_as_factory(13i)
        ]);

        let parent_metafactory = metafactory(
            |items: Vec<int>|
                items.into_iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<String>>()
                    .connect(", ")
        );

        let parent_getter = parent_metafactory
            .new(vec![
                container.new_factory()
            ]).ok().unwrap()
            .as_factory_of::<String>().unwrap()
        ;

        assert_eq!(parent_getter.take(), "5, 13");
    }
}
