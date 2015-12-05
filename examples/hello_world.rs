extern crate di;

use di::{ Deps, Parent, WithAll };
use di::extension::{ On, OnMany };

struct A;

fn main() {
    let mut deps = Deps::new();

    deps.on(|parent: Parent<i32>| println!("hello {:?}", parent));
    deps.on(|_: Parent<i32>| true);
    deps.on(|_: Parent<i32>| false);
    deps.on(|mut parent: Parent<bool>| {
        println!("bool {:?}!", parent);
        *parent = false;
        A
    });
    deps.on_2(|val: &i32, flag: &bool| {

    });
    deps.on(|_: Parent<A>| {
        println!("A was created!");
    });

    5.with_all(&deps);
}
