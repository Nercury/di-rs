use term;
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
        }
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
