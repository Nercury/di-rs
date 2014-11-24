/// Base factory trait for creation of a value.
#[unstable]
pub trait Getter<T> {
    fn get(&self) -> T;
}

#[stable]
pub struct GetterWrap<'a, T> {
    getter: Box<Getter<T> + 'a>,
}

#[stable]
impl<'a, T> GetterWrap<'a, T> {
    pub fn new(getter: Box<Getter<T> + 'a>) -> GetterWrap<'a, T> {
        GetterWrap::<T> {
            getter: getter,
        }
    }

    pub fn get(&self) -> T {
        self.getter.get()
    }
}
