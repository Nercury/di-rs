/// Base factory trait for creation of a value.
#[unstable]
pub trait Getter<T> {
    fn get(&self) -> T;
}

/// Concrete wrapper for getter trait to allow use in Box<Any>.
#[stable]
pub struct GetterWrap<'a, T> {
    getter: Box<Getter<T> + 'a>,
}

#[stable]
impl<'a, T> GetterWrap<'a, T> {
    /// Create a new getter wrap for Getter.
    pub fn new(getter: Box<Getter<T> + 'a>) -> GetterWrap<'a, T> {
        GetterWrap::<T> {
            getter: getter,
        }
    }

    /// Get a value using inner getter.
    pub fn get(&self) -> T {
        self.getter.get()
    }
}
