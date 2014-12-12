use std::any::{ Any, AnyMutRefExt };
use std::boxed::BoxAny;
use metafactory::factory::{ Factory, Getter };

/// Proxy for configuring factory list
/// without caring about the type used.
pub struct FactoryContainer<'a> {
    any_getter: Box<Any>,
    do_push_items: |&mut Box<Any>, Vec<Box<Any>>|:'a -> (),
    do_new_factory: |&mut Box<Any>|:'a -> Box<Any>, // Don't worry, it's like Javascript ;)
}

impl<'a> FactoryContainer<'a> {
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
