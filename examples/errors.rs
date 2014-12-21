extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();

    registry
        .one("duplicate", |input: int| input)
        .add_arg("arg")
        .insert()
    ;
    registry
        .one("duplicate", |i: int| i)
        .add_arg("arg")
        .insert()
    ;
    registry
        .one("duplicate", |_input: &'static str| 4i)
        .add_arg("what")
        .insert()
    ;
    registry
        .one("duplicate", |input: int, _flag: bool| input)
        .with_args(&["arg", "cc"])
        .insert()
    ;

    registry
        .one("too_many_dependencies", || "output")
        .with_args(&["a", "b", "c"])
        .insert()
    ;

    registry
        .one("missing_dependencies", |_ok: i32, _a: int, _b: bool, _c: Vec<String>| "output")
        .add_arg("ok")
        .insert()
    ;

    registry
        .one_of("miracles", "missing_miracle", |reason: &'static str| reason)
        .add_arg("miracle_reason")
        .insert()
    ;

    // Should print nice error.

    match registry.compile() {
        Ok(_) => panic!("But I expected errors!"),
        Err(errors) => {
            di::error_printer::pretty_print(&errors);
        }
    }
}
