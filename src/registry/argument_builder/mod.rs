pub struct ArgumentBuilder {
    pub arg_sources: Vec<String>,
}

impl ArgumentBuilder {
    pub fn new() -> ArgumentBuilder {
        ArgumentBuilder {
            arg_sources: Vec::new(),
        }
    }

    pub fn set_arg_sources<'r>(&'r mut self, arg_sources: &[&str]) {
        self.arg_sources.truncate(0);
        for str in arg_sources.iter() {
            self.arg_sources.push(str.to_string());
        }
    }

    pub fn set_arg_source<'r>(&'r mut self, arg_source: &str) {
        self.arg_sources.truncate(0);
        self.arg_sources.push(arg_source.to_string());
    }

    pub fn push_arg_source<'r>(&'r mut self, arg_source: &str) {
        self.arg_sources.push(arg_source.to_string());
    }
}
