extern crate di;

fn main() {
    let mut registry = di::registry::Registry::new();
    registry
        .has_many::<|String|:'static -> String>("printers")
    ;

    registry
        .one_of("printers", "first_printer", |name: String| {
            |val: &str| println!("{}, {}", name, val);
        })
        .with_arg("first_name")
        .insert()
    ;

    registry
        .one_of("printers", "second_printer", |name: &'static str| {
            |val: &str| println!("{}, {}", name, val);
        })
        .add_arg("second_name")
        .insert()
    ;

    registry.insert_with_arg_one("output", "printers", |printers: Vec<|String|:'static -> String>| {
        let mut mut_printers = printers;
        mut_printers.iter_mut()
            .map(|p| (*p)("Text".to_string()))
            .collect::<Vec<String>>()
            .connect(" - ")
    });

    registry.insert_one("first_name", "Printer One");
    registry.one("second_name", "Printer Second").insert();

    let maybe_container = registry.compile();
    //
    // let source = container.source_of_many::<|| -> ()>("printers");
    // let printers = source.new();
    //
    // for printer in printers.iter() {
    //     (*printer)("Hi");
    // }
}
