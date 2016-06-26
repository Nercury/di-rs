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

    deps.attach(|_: &Deps, _: &mut Alpha| Ok(AlphaExtension::new()));
    deps.attach(|_: &Deps, _: &mut AlphaExtension| Ok(AlphaExtensionExtension::new()));

    let dep_refs = Arc::new(deps);

    let a = thread::spawn({
        let a_deps = dep_refs.clone();
        move || {
            a_deps.create(Alpha::new()).unwrap();
        }
    });

    let b = thread::spawn({
        let b_deps = dep_refs.clone();
        move || {
            b_deps.create(Alpha::new()).unwrap();
        }
    });

    b.join().unwrap();
    a.join().unwrap();
}
