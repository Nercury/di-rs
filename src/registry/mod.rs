use std::any::Any;
use metafactory::{ ToMetaFactory, MetaFactory };

pub struct Registry {
    aa: int,
}

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
}

pub struct OneOfParams<'a> {
    collection_id: String,
    id: String,
    value: Box<MetaFactory + 'a>,
    arg_builder: ArgumentBuilder,
}

pub struct OneOfCandidate<'a> {
    finalizer: &'a mut (OneOfFinalizer + 'a),
    params: OneOfParams<'a>,
}

impl<'a> OneOfCandidate<'a> {
    pub fn new(
        params: OneOfParams<'a>,
        finalizer: &'a mut OneOfFinalizer
    )
    -> OneOfCandidate<'a>
    {
        OneOfCandidate {
            finalizer: finalizer,
            params: params,
        }
    }

    pub fn with_args(
        mut self,
        arg_sources: &[&str]
    )
    -> OneOfCandidate<'a>
    {
        self.params.arg_builder.set_arg_sources(arg_sources);
        self
    }

    pub fn insert(self) -> &'a mut (OneOfFinalizer + 'a) {
        let finalizer = self.finalizer;
        finalizer.finalize_one_of(self.params);
        finalizer
    }
}

pub trait OneOfFinalizer {
    fn finalize_one_of<'a>(&mut self, params: OneOfParams<'a>);
}

impl OneOfFinalizer for Registry {
    fn finalize_one_of<'a>(&mut self, params: OneOfParams<'a>) {
        self.insert_with_args_one_of(
            params.collection_id.as_slice(),
            params.id.as_slice(),
            params.value,
            params.arg_builder.arg_sources
        );
    }
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            aa: 3,
        }
    }

    pub fn one_of<'r, T: ToMetaFactory>(&'r mut self, collection_id: &str, id: &str, value: T) -> OneOfCandidate<'r> {
        OneOfCandidate::new(
            OneOfParams {
                collection_id: collection_id.to_string(),
                id: id.to_string(),
                value: value.to_metafactory(),
                arg_builder: ArgumentBuilder::new(),
            },
            self
        )
    }

    fn insert_with_args_one_of<'r>(&mut self, collection_id: &str, id: &str, value: Box<MetaFactory + 'r>, args: Vec<String>) {

    }
}
