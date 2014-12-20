use registry::error;

mod pretty_terminal;

pub trait ErrorWriter {
    fn error(&mut self, m: &str);
    fn definition(&mut self, m: &str);
    fn module(&mut self, m: &str);
    fn typename(&mut self, m: &str);
    fn number(&mut self, m: &str);
    fn operator(&mut self, m: &str);
    fn layout(&mut self, m: &str);
    fn text(&mut self, m: &str);
    fn eol(&mut self);
    fn flush(&mut self);
}

pub fn pretty_print(errors: &Vec<error::CompileError>) {
    let mut writer = pretty_terminal::PrettyTerminalOutput::new();
    for error in errors.iter() {
        pretty_print_single(&mut writer, error);
    }
}

pub fn pretty_print_single(w: &mut ErrorWriter, error: &error::CompileError) {
    match error {
        &error::CompileError::DuplicateDefinitions(ref error) => {
            w.error("Error: ");
            w.text("the name ");
            w.definition(
                error.aliases.values().next().unwrap()
                    .definition.id.as_slice()
            );
            w.text(" is not unique:");
            w.eol();

            for (_, duplicate) in error.aliases.iter() {
                w.layout(" |> ");
                w.number(format!("{}", duplicate.count).as_slice());
                w.text(" of ");
                pretty_print_definition(w, &duplicate.definition);
                w.eol();
            }
        },
        &error::CompileError::ArgumentCountMismatch(ref error) => {
            w.error("Error: ");
            w.text("the definition ");
            w.definition(
                error.id.as_slice()
            );
            if error.arg_sources.len() > error.arg_types.len() {
                let unecessary_sources: Vec<String> = error.arg_sources.iter()
                    .skip(error.arg_types.len())
                    .map(|r| r.clone())
                    .collect();
                let len = unecessary_sources.len();

                if len == 1 {
                    w.text(" does not need extra argument ");
                    w.definition(
                        unecessary_sources[0].as_slice()
                    );
                } else {
                    let heads = unecessary_sources[0 .. len - 2];
                    let middle = &unecessary_sources[len - 2];
                    let tail = &unecessary_sources[len - 1];
                    w.text(" does not need extra arguments ");
                    for head in heads.iter() {
                        w.definition(head.as_slice());
                        w.text(", ");
                    }
                    w.definition(middle.as_slice());
                    w.text(" and ");
                    w.definition(tail.as_slice());
                }
            } else {
                w.text(" has ");
                w.number(format!("{}", error.arg_types.len() - error.arg_sources.len()).as_slice());
                w.text(" undefined dependencies:");

                pretty_print_missing_dependencies(w, error);
            }
            w.eol();
        },
    }
}

pub fn pretty_print_missing_dependencies(w: &mut ErrorWriter, error: &error::ArgumentCountMismatch) {
    let arg_sourcesc = error.arg_sources.len();

    for (source, typedef) in error.arg_sources.iter().zip(error.arg_types.iter()) {
        w.eol();
        w.layout(" |> ");

        w.definition(source.as_slice());
        w.text(": ");
        w.typename(typedef.get_str());
    }

    for typedef in error.arg_types[arg_sourcesc..error.arg_types.len()].iter() {
        w.eol();
        w.layout(" |> ");

        w.error("?");
        w.text(": ");
        w.typename(typedef.get_str());
    }
}

pub fn pretty_print_definition(w: &mut ErrorWriter, definition: &error::Definition) {
    w.definition(definition.id.as_slice());

    let argc = definition.args.len();
    if argc > 0 {
        w.text(" (");

        let mut index = 0;

        for arg in definition.args.iter() {
            w.definition(arg.source.as_slice());
            w.text(": ");
            w.typename(arg.typedef.get_str());

            index += 1;
            if index < argc {
                w.text(", ");
            }
        }

        w.text(")");
    }

    w.operator(" -> ");
    w.typename(definition.typedef.get_str());
}
