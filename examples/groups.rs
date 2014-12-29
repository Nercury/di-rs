extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();

    // Can insert anonymous cloneable value that belongs to group "integers".
    registry.insert_one_of("integers", 1i);

    // Can insert named value that belongs to "integers".
    registry
        .one_of("integers", 2i)
        .with_id("a")
        .insert();

    // Can insert value that belongs to "integers" and depends on another value.
    registry
        .one_of("integers", |a: int| a + 1)
        .with_arg("a")
        .insert();

    match registry.compile() {
        Ok(container) => {
            if let Some(integer_factory) = container.get::<Vec<int>>("integers") {
                for i in integer_factory.take().iter() {
                    println!("Value: {}", i);
                }
            }
        },
        Err(errors) => di::error_printer::pretty_print(&errors),
    }
}
