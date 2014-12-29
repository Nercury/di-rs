extern crate di;
extern crate metafactory;

fn main() {
    let mut registry = di::registry::Registry::new();

    // Can define simple clonable values in registry.
    registry.insert_one("a", 5i);
    registry.insert_one("b", 4i);

    // Can define a value that depends on two other.
    registry
        .one(
            "sum",
            |a: int, b: int|
            a + b
        )
        .with_args(&["a", "b"])
        .insert();

    // Can reuse same dependencies.
    registry
        .one(
            "difference",
            |a: int, b: int|
            a - b
        )
        .with_args(&["a", "b"])
        .insert();

    // Can define more complex things, like a function.
    registry.insert_one("into_string", || -> Box<|int|:'static -> String> {
        box |value: int| value.to_string()
    });

    // Can use them all as dependencies.
    registry
        .one(
            "results",
            |sum: int, difference: int, into_string: Box<|value: int|:'static -> String>| {
                vec![(*into_string)(sum), (*into_string)(difference)]
            }
        )
        .with_args(&["sum", "difference", "into_string"])
        .insert();

    match registry.compile() {
        Ok(container) => {
            if let Some(results) = container.get::<Vec<String>>("results") {
                println!("results: {}", results.take().connect(", "));
            }
        },
        Err(errors) => {
            di::error_printer::pretty_print(&errors);
        }
    }
}
