use term;
use super::registry::error;

pub fn pretty_print(errors: &Vec<error::CompileError>) {
    for error in errors.iter() {
        pretty_print_single(error);
    }
}

pub fn pretty_print_single(error: &error::CompileError) {
    match error {
        &error::CompileError::DuplicateDefinitions(ref error) => {
            let mut t = term::stdout().unwrap();

            t.fg(term::color::BRIGHT_RED).unwrap();
            (write!(t, "Error: ")).unwrap();
            t.reset().unwrap();

            t.fg(term::color::BRIGHT_YELLOW).unwrap();
            (write!(t, "{}", format_definition(&error.added))).unwrap();
            t.reset().unwrap();

            (write!(t, " hides previously defined:")).unwrap();

            t.flush();

            println!("");

            for (_, duplicate) in error.aliases.iter() {
                t.fg(term::color::BRIGHT_YELLOW).unwrap();
                (write!(t, " |> ")).unwrap();
                t.reset().unwrap();

                t.fg(term::color::MAGENTA).unwrap();
                (write!(t, "{}", duplicate.count)).unwrap();
                t.reset().unwrap();

                (write!(t, " of ")).unwrap();

                t.fg(term::color::BRIGHT_YELLOW).unwrap();
                (write!(t, "{}", format_definition(&duplicate.definition))).unwrap();
                t.reset().unwrap();

                t.flush();

                println!("");
            }
        }
    }
}

pub fn format_definition(definition: &error::Definition) -> String {
    let mut result = String::new();
    result.push_str(definition.id.as_slice());

    let argument_str = definition.args.iter()
        .map(|&:a| {
            let mut argument_with_type = String::new();
            argument_with_type.push_str(a.source.as_slice());
            argument_with_type.push_str(": ");
            argument_with_type.push_str(a.typedef.get_str());
            argument_with_type
        })
        .collect::<Vec<String>>()
        .connect(", ");

    if argument_str.len() > 0 {
        result.push_str(" (");
        result.push_str(argument_str.as_slice());
        result.push(')');
    }

    result.push_str(" -> ");
    result.push_str(definition.typedef.get_str());

    result
}
