extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();

    registry.insert_one("arg", 12i);

    // Define several duplicates.

    registry
        .one("item", |input: int| input)
        .add_arg("arg")
        .insert()
    ;
    registry
        .one("item", |i: int| i)
        .add_arg("arg")
        .insert()
    ;
    registry
        .one("item", |_input: &'static str| 4i)
        .add_arg("what")
        .insert()
    ;
    registry
        .one("item", |input: int, _flag: bool| input)
        .with_args(&["arg", "cc"])
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
