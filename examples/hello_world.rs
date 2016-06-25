extern crate di;

use di::Deps;
use std::thread;
use std::sync::Arc;

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

    deps.on(|_: &Deps, _: &mut Alpha| AlphaExtension::new());
    deps.on(|_: &Deps, _: &mut AlphaExtension| AlphaExtensionExtension::new());

    let dep_refs = Arc::new(deps);

    let a = thread::spawn({
        let a_deps = dep_refs.clone();
        move || {
            a_deps.create_for(Alpha::new());
        }
    });

    let b = thread::spawn({
        let b_deps = dep_refs.clone();
        move || {
            b_deps.create_for(Alpha::new());
        }
    });

    b.join().unwrap();
    a.join().unwrap();
}
