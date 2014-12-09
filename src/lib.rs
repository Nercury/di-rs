extern crate typedef;
extern crate metafactory;

pub mod registry;

pub struct Container {
    aa: int,
}


impl Container {
    pub fn new() -> Container {
        Container {
            aa: 4,
        }
    }
}
