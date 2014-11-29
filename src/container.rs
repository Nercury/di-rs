use std::any::{ Any, AnyRefExt };
use std::collections::HashMap;

use super::{ GetterErr, GetterErrKind, GetterTypeErr, DefinitionTypeErr, Registry };
use super::definition::{ TypeDef };
use super::getter::{ GetterWrap };

struct ContainerItem {
    typedef: TypeDef,
    getter: Box<Any>,
}

pub struct Container {
    getters: HashMap<String, ContainerItem>,
}

impl Container {
    /// Create container from registry definitions.
    pub fn from_registry(registry: &Registry) -> Result<Container, Vec<GetterErr>> {
        let mut c = Container {
            getters: HashMap::new(),
        };

        let mut errors = Vec::<GetterErr>::new();

        for name in registry.all_names().iter() {
            match registry.any_getter_for(name.as_slice()) {
                Ok((typedef, getter)) => {
                    c.getters.insert(name.clone(), ContainerItem { typedef: typedef, getter: getter });
                },
                Err(e) => { errors.push(e); },
            };
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(c)
    }

    /// Get a getter for a definition.
    pub fn getter_for<'r, T: 'static>(
        &'r self, name: &str
    )
        -> Result<&'r GetterWrap<T>, GetterErr>
    {
        let maybe_item = self.getters.get(name);
        match maybe_item {
            Some(item) => {
                if !item.typedef.is::<T>() {
                    return Err(GetterErr::new(
                        GetterErrKind::DefinitionTypeMismatch(DefinitionTypeErr::new(TypeDef::of::<T>(), item.typedef)),
                        name
                    ));
                }

                match item.getter.downcast_ref::<GetterWrap<T>>() {
                    Some(getter_wrap) => Ok(getter_wrap),
                    None => {
                        Err(GetterErr::new(
                            GetterErrKind::GetterTypeMismatch(GetterTypeErr::new(TypeDef::of::<GetterWrap<T>>())),
                            name
                        ))
                    },
                }
            },
            None => Err(GetterErr::new(GetterErrKind::NotFound, name)),
        }
    }
}
