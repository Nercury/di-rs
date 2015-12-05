extern crate di;

use di::{ Deps, WithAll };
use di::extension::On;

fn main() {
    let mut deps = Deps::new();

    deps.on(|_, parent: &i32| println!("hello {:?}", parent));
    deps.on(|deps, _: &i32| true.with_all(deps));
    deps.on(|deps, _: &i32| false.with_all(deps));
    deps.on(|_, parent: &bool| println!("bool {:?}!", parent));

    5.with_all(&deps);
}
