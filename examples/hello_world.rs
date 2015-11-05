extern crate di;

use di::{ Dependencies, WithAll };

fn main() {
    let mut deps = Dependencies::new();

    deps.on_one(|_, parent: &i32| println!("hello {:?}", parent));
    deps.on_one(|deps, _: &i32| true.with_all(deps));
    deps.on_one(|deps, _: &i32| false.with_all(deps));
    deps.on_one(|_, parent: &bool| println!("bool {:?}!", parent));

    5.with_all(&deps);
}
