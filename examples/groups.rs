extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();



    // Should print nice error.

    match registry.compile() {
        Ok(_) => {
            println!("Might work.");
        },
        Err(errors) => {
            di::error_printer::pretty_print(&errors);
        }
    }
}
