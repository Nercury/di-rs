extern crate di;

use di::{ Deps, Parent, WithAll };
use di::extension::{ On, OnMany };
use std::thread;
use std::sync::Arc;
use std::cell::RefCell;

struct Alpha;
struct AlphaExtension;
struct AlphaExtensionExtension;

impl Alpha {
    pub fn new() -> Alpha {
        println!("Alpha created");
        Alpha
    }
}

impl Drop for Alpha {
    fn drop(&mut self) {
        println!("Alpha deleted");
    }
}

impl AlphaExtension {
    pub fn new() -> AlphaExtension {
        println!("Alpha Extension created");
        AlphaExtension
    }
}

impl Drop for AlphaExtension {
    fn drop(&mut self) {
        println!("Alpha Extension deleted");
    }
}

impl AlphaExtensionExtension {
    pub fn new() -> AlphaExtensionExtension {
        println!("Alpha ExtensionExtension created");
        AlphaExtensionExtension
    }
}

impl Drop for AlphaExtensionExtension {
    fn drop(&mut self) {
        println!("Alpha ExtensionExtension deleted");
    }
}

fn main() {
    let mut deps = Deps::new();

    deps.on(|_: Parent<Alpha>| AlphaExtension::new());
    deps.on(|_: Parent<AlphaExtension>| AlphaExtensionExtension::new());

    let dep_refs = Arc::new(deps);

    let a = thread::spawn({
        let a_deps = dep_refs.clone();
        move || {
            Alpha::new().with_all(a_deps);
        }
    });

    let b = thread::spawn({
        let b_deps = dep_refs.clone();
        move || {
            Alpha::new().with_all(b_deps);
        }
    });

    b.join().unwrap();
    a.join().unwrap();
}
