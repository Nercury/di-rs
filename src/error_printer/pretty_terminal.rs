use term;
use super::ErrorWriter;

pub struct PrettyTerminalOutput {
    color_error: Option<term::color::Color>,
    color_definition: Option<term::color::Color>,
    color_module: Option<term::color::Color>,
    color_typename: Option<term::color::Color>,
    color_number: Option<term::color::Color>,
    color_operator: Option<term::color::Color>,
    color_layout: Option<term::color::Color>,

    color: Option<term::color::Color>,

    t: Box<term::Terminal<term::WriterWrapper>+Send>,
}

impl PrettyTerminalOutput {
    pub fn new() -> PrettyTerminalOutput {
        PrettyTerminalOutput {
            color_error: Some(term::color::BRIGHT_RED),
            color_definition: Some(term::color::BRIGHT_YELLOW),
            color_module: Some(term::color::BRIGHT_BLUE),
            color_typename: Some(term::color::CYAN),
            color_number: Some(term::color::MAGENTA),
            color_operator: Some(term::color::BRIGHT_WHITE),
            color_layout: Some(term::color::YELLOW),
            color: None,
            t: term::stdout().unwrap(),
        }
    }

    pub fn set_color(&mut self, color: Option<term::color::Color>) {
        if self.color != color {
            self.color = color;
            if let Some(c) = color {
                self.t.fg(c).unwrap();
            } else {
                self.t.reset().unwrap();
            }
        }
    }
}

impl ErrorWriter for PrettyTerminalOutput {

    fn error(&mut self, m: &str) {
        let new_color = self.color_error.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn definition(&mut self, m: &str) {
        let new_color = self.color_definition.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn module(&mut self, m: &str) {
        let new_color = self.color_module.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn typename(&mut self, m: &str) {
        let new_color = self.color_typename.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn number(&mut self, m: &str) {
        let new_color = self.color_number.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn operator(&mut self, m: &str) {
        let new_color = self.color_operator.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn layout(&mut self, m: &str) {
        let new_color = self.color_layout.clone();
        self.set_color(new_color);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn text(&mut self, m: &str) {
        self.set_color(None);
        (write!(self.t, "{}", m)).unwrap();
    }

    fn eol(&mut self) {
        (writeln!(self.t, "")).unwrap();
    }

    fn flush(&mut self) {
        self.t.flush();
    }
}
