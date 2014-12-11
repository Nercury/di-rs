//use std::cmp::{ Eq };
//use std::hash;
//use std::collections::{ HashMap, HashSet };

use typedef::TypeDef;
use metafactory::MetaFactory;

struct Group {
    id: String,
    typedef: TypeDef,
}

struct Definition<'a> {
    origin: String,
    id: String,
    group: Option<Group>,
    metafactory: Box<MetaFactory + 'a>,
}

pub struct Definitions<'a> {
    items: Vec<Definition<'a>>,
}

impl<'a> Definitions<'a> {
    pub fn new() -> Definitions<'a> {
        Definitions {
            items: Vec::new(),
        }
    }

    pub fn define(
        &mut self,
        origin: &str,
        id: &str,
        metafactory: &str
    ) {

    }

    pub fn define_group(
        &mut self,
        origin: &str,
        id: &str,
        metafactory: &str
    ) {

    }
}

//
// impl<'a> PartialEq for Definition<'a> {
//     fn eq(&self, other: &Definition) -> bool {
//         self.id.eq(&other.id)
//     }
// }
//
// impl<'a, H: hash::Writer> hash::Hash<H> for Definition<'a> {
//     fn hash(&self, hasher: &mut H) {
//         self.id.hash(hasher);
//     }
// }
//
// impl<'a> Eq for Definition<'a> { }
//
//
// impl PartialEq for Group {
//     fn eq(&self, other: &Group) -> bool {
//         self.id.eq(&other.id)
//     }
// }
//
// impl<H: hash::Writer> hash::Hash<H> for Group {
//     fn hash(&self, hasher: &mut H) {
//         self.id.hash(hasher);
//     }
// }
//
// impl Eq for Group { }
