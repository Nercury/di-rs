extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();
    registry
        .one_of("printers", "first_printer", |name: String| {
            |val: &str| println!("{}, {}", name, val);
        })
        .with_args(&["first_name"])
        .insert()
    ;

    registry.insert_with_args_one("output", &["printers"], |printers: Vec<|String| -> String>| {
        let mut mut_printers = printers;
        mut_printers.iter_mut()
            .map(|p| (*p)("Text".to_string()))
            .collect::<Vec<String>>()
            .connect(" - ")
    });

    // registry
    //     .one_of("printers", |name: &str| {
    //         |val: &str| println!("{}, {}", name, val);
    //     })
    //     .with_arg("second_name")
    //     .insert()
    // ;
    //
    // registry.insert_one("first_name", "Printer One");
    // registry.insert_one("second_name", "Printer Second");
    //
    // let container = di::Container::new(registry);
    //
    // let source = container.source_of_many::<|| -> ()>("printers");
    // let printers = source.new();
    //
    // for printer in printers.iter() {
    //     (*printer)("Hi");
    // }
}
