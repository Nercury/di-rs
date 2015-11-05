/*!
<a href="https://github.com/Nercury/di-rs">
    <img style="position: absolute; top: 0; left: 0; border: 0;" src="https://s3.amazonaws.com/github/ribbons/forkme_left_green_007200.png" alt="Fork me on GitHub">
</a>
<style>.sidebar { margin-top: 53px }</style>
*/

/*!

# Dependency Injection container for Rust

This page contains an abstract (although with few examples) overview
of this dependency injection container. To see more fine-grained
usage examples without much prose, browse the [Registry](registry/struct.Registry.html)
documentation.

# Overview

## Declarative feature configuration

If a project needs X, Y and Z features, it should be
possible to declare a list of these features, and the features should
discover each other and configure themselves.

## Dependency injection (DI) as a method

First of all, while __dependency injection__ term is just a fancy name for
"passing arguments to constructor", it also communicates a specific
intent: __the user of these arguments does not care about
the way they were created__.

The simpliest form of dependency abstraction is use of trait objects
as construction arguments.

# Discussion

This is opionated part.

## Why did this library happen

I wanted to implement DI mechanism for Rust because I had great success
with it before. The idea of "container" and "registering" definitions in
it [comes from Symfony2][symfony2-container-component] framework.

However, I intentionaly chose to avoid implementing things the same way
Symfony2 does.

For example, the factories do not return singletons by default. However,
they can be easily added using clonable value over `Rc` wrapper.

The initialization mechanism requires a closure or clonable value.
If you do not like that, well, it is possible to implement
`metafactory::ToMetafactory` trait for anything you would like to use
for value construction.

It was possible to use container itself as an argument for service construction.
It was highly discouraged anti-pattern. So, no such thing here.

The container was highly coupled to the way configuration is loaded. I
consider configuration not a concern of this library, therefore nothing
like that will be implemented.

There was a way to register "passes" for compilation and let various bundles
to modify the registry before the container is actually built. I might
consider adding this.

There is no "priority" yet for aggregates. I am a bit weary of this "feature"
based on the hours spent hunting down services that load in wrong order.
But I am considering that ability to explicitly "override" some definition
might be ok.

There is one other pattern that is a bit burried in Symfony2 di: it is the
ability to "tag" services and then have the container inject all of them
based on that "tag". [Using that is not straightforward][symfony2-tagged-services].
However, I found that it was the key for decoupling features properly, so I chose
to make it __very__ easy to use the equivalent functionality in this library.
That's where `one_of` method with factory `Aggregate` came from.

[symfony2-container-component]: http://symfony.com/doc/current/components/dependency_injection/introduction.html
[symfony2-tagged-services]: http://symfony.com/doc/current/components/dependency_injection/tags.html

## Higher-level features and dependencies

As I briefly mentioned before, my motivation for creating this library
is managing the feature decoupling. Now I will talk
what I mean by "feature".

The worst case scenario in huge application happens when the low level tools
that need to be reliable become coupled to some idiosyncrasies of the
project.

What user (project) wants (requires) does not usually live well with the idea
of having a stable code. Likewise, replacing crucial library does not live
well with the stability the user (project) expects (needs).

Ultimately, these things should live in separate libraries. To distinguish
these libraries, I call code that is responsible for project a "feature", and
the low-level tools simply "libraries".

This `di` library is intended to be one of the ways to manage separation
of features from libraries. The `di` should live at the project-level.
It should register the available libraries to do what needs to be done,
and additionaly provide the "extension" points.

Then, the functionality that is specific only to current project should
be implemented as extension (over `one_of`) or replacement (over `override`
(which is not done yet :P)).

## Usability in actual plugins

This needs to be investigated. Might be fun.

*/

use std::any::{ Any, TypeId };
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::{ Deref, DerefMut };

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

pub trait WithAll<T> {
    fn with_all(self, deps: &Dependencies) -> Scope<T>;
}

impl<T: Any> WithAll<T> for T {
    fn with_all(self, deps: &Dependencies) -> Scope<T> {
        deps.create_dependencies(self)
    }
}

pub struct Dependencies {
    constructors: HashMap<TypeId, Vec<Box<Fn(&Dependencies, &Any) -> Box<Any>>>>,
}

impl Dependencies {
    pub fn new() -> Dependencies {
        Dependencies {
            constructors: HashMap::new()
        }
    }

    pub fn create_dependencies<P: Any>(&self, obj: P) -> Scope<P> {
        match self.constructors.get(&TypeId::of::<P>()) {
            Some(list) => {
                let dependencies: Vec<_> = list.iter()
                    .map(|any_constructor| any_constructor(&self, &obj))
                    .collect();

                Scope { obj: obj, childs: dependencies }
            },
            None => Scope { obj: obj, childs: vec![] },
        }
    }

    pub fn on_one<P, C, F>(&mut self, constructor: F)
        where
            P: 'static + Any, C: 'static + Any,
            F: for<'r> Fn(&'r Dependencies, &P) -> C + 'static
    {
        self.upsert(TypeId::of::<P>(), any_constructor(constructor));
    }

    fn upsert(
        &mut self,
        type_id: TypeId,
        any_constructor: Box<Fn(&Dependencies, &Any) -> Box<Any>>
    ) {
        match self.constructors.entry(type_id) {
            Entry::Occupied(mut list) => {
                list.get_mut().push(any_constructor);
            },
            Entry::Vacant(e) => {
                e.insert(vec![any_constructor]);
            },
        };
    }

}

fn any_constructor<P, C, F>(constructor: F) -> Box<Fn(&Dependencies, &Any) -> Box<Any>>
    where F: for<'r> Fn(&'r Dependencies, &P) -> C + 'static, P: 'static + Any, C: 'static + Any
{
    Box::new(move |deps: &Dependencies, parent: &Any| -> Box<Any> {
        let concrete_parent = parent.downcast_ref::<P>().unwrap();
        let child = constructor(deps, concrete_parent);
        Box::new(child)
    })
}
