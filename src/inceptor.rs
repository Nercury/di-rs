//! It incepts.

use std::sync::{Arc, Mutex};
use std::any::Any;
use std::mem;
use Result;

pub struct Inceptor<T1, T2> {
    d1: Vec<Option<Arc<Mutex<T1>>>>,
    d2: Vec<Option<Arc<Mutex<T2>>>>,
    used_size1: usize,
    used_size2: usize,
    constructor: Arc<Fn(&mut T1, &mut T2) -> Result<Option<Box<Any>>> + Send + Sync>,
}

unsafe impl<T1, T2> Send for Inceptor<T1, T2> {}
unsafe impl<T1, T2> Sync for Inceptor<T1, T2> {}

fn insert_into_vec<T>(data: &mut Vec<Option<Arc<Mutex<T>>>>,
                      value: Arc<Mutex<T>>,
                      used_size: &mut usize)
                      -> usize {
    for (i, item) in data.iter_mut().enumerate() {
        if item.is_none() {
            let mut tmp = Some(value);
            mem::swap(&mut tmp, item);
            if i + 1 > *used_size {
                *used_size = i + 1;
            }
            return i;
        }
    }
    let index = data.len();
    data.push(Some(value));
    if data.len() > *used_size {
        *used_size = data.len();
    }
    index
}

impl<T1: Any, T2: Any> Inceptor<T1, T2> {
    pub fn new<F: 'static>(constructor: F) -> Inceptor<T1, T2>
        where F: Fn(&mut T1, &mut T2) -> Result<Option<Box<Any>>> + Send + Sync
    {
        Inceptor {
            d1: Vec::new(),
            d2: Vec::new(),
            used_size1: 0,
            used_size2: 0,
            constructor: Arc::new(constructor),
        }
    }

    fn invoke(&mut self, i1: usize, i2: usize) -> Result<Option<Box<Any>>> {
        let val1: &mut Arc<Mutex<T1>> = match *self.d1
            .get_mut(i1)
            .expect("expected to find i1") {
            Some(ref mut val) => val,
            None => unreachable!("expected i1 to exist at slot"),
        };
        let val2 = match *self.d2
            .get_mut(i2)
            .expect("expected to find i2") {
            Some(ref mut val) => val,
            None => unreachable!("expected i2 to exist at slot"),
        };
        let mut locked1 = val1.lock().expect("expected to lock val1");
        let mut locked2 = val2.lock().expect("expected to lock val2");
        (self.constructor)(&mut locked1, &mut locked2)
    }

    pub fn incept_1(&mut self, parent: Arc<Mutex<T1>>) -> Result<(usize, Vec<Box<Any>>)> {
        let id = insert_into_vec(&mut self.d1, parent, &mut self.used_size1);
        let mut results = Vec::new();
        for i2 in 0..self.d2.len() {
            if !self.d2[i2].is_none() {
                match try!(self.invoke(id, i2)) {
                    Some(res) => results.push(res),
                    None => (),
                }
            }
        }
        Ok((id, results))
    }

    pub fn incept_2(&mut self, parent: Arc<Mutex<T2>>) -> Result<(usize, Vec<Box<Any>>)> {
        let id = insert_into_vec(&mut self.d2, parent, &mut self.used_size2);
        let mut results = Vec::new();
        for i1 in 0..self.d1.len() {
            if !self.d1[i1].is_none() {
                match try!(self.invoke(i1, id)) {
                    Some(res) => results.push(res),
                    None => (),
                }
            }
        }
        Ok((id, results))
    }

    pub fn destroy_1(&mut self, id: usize) {
        if id + 1 == self.used_size1 {
            self.used_size1 -= 1;
        }
        let mut tmp: Option<Arc<Mutex<T1>>> = None;
        mem::swap(&mut tmp,
                  self.d1
                      .get_mut(id)
                      .expect(&format!("expected to find destroy_1 value {:?}", id)));
    }
    pub fn destroy_2(&mut self, id: usize) {
        if id + 1 == self.used_size2 {
            self.used_size2 -= 1;
        }
        let mut tmp: Option<Arc<Mutex<T2>>> = None;
        mem::swap(&mut tmp,
                  self.d2
                      .get_mut(id)
                      .expect(&format!("expected to find destroy_2 value {:?}", id)));
    }
}

pub struct Destructor<T1: Any, T2: Any> {
    /// Inceptor to clean
    inceptor: Arc<Mutex<Inceptor<T1, T2>>>,
    /// Which method, 1 or 2
    index: usize,
    /// Id to clean
    id: usize,
}

impl<T1: Any, T2: Any> Destructor<T1, T2> {
    pub fn new(inceptor: Arc<Mutex<Inceptor<T1, T2>>>,
               index: usize,
               id: usize)
               -> Destructor<T1, T2> {
        Destructor {
            inceptor: inceptor,
            index: index,
            id: id,
        }
    }
}

impl<T1: Any, T2: Any> Drop for Destructor<T1, T2> {
    fn drop(&mut self) {
        if self.index == 1 {
            self.inceptor.lock().expect("failed to lock").destroy_1(self.id);
        } else if self.index == 2 {
            self.inceptor.lock().expect("failed to lock").destroy_2(self.id);
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use super::*;

    fn count_not_none<T>(data: &mut Vec<Option<Arc<Mutex<T>>>>) -> usize {
        data.iter().filter(|v| !v.is_none()).count()
    }

    #[test]
    fn should_register_and_destroy_first_arg() {
        let mut ic = Inceptor::<i32, bool>::new(|_a, _b| Ok(Some(Box::new(42))));
        let (id, instances) = ic.incept_1(Arc::new(Mutex::new(11)))
            .expect("failed to incept first arg");

        assert_eq!(instances.len(), 0);
        assert_eq!(count_not_none(&mut ic.d1), 1);

        ic.destroy_1(id);

        assert_eq!(count_not_none(&mut ic.d1), 0);
    }
}
