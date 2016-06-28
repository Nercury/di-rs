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

    pub fn new_with_ignored_return_val<C, F>(constructor: F) -> Inceptor<T1, T2>
        where C: 'static + Any,
              F: for<'r> Fn(&mut T1, &mut T2) -> Result<C> + 'static + Send + Sync
    {
        Self::new(move |p1: &mut T1, p2: &mut T2| -> Result<Option<Box<Any>>> {
            try!(constructor(p1, p2));
            Ok(None)
        })
    }

    pub fn new_with_return_val<C, F>(constructor: F) -> Inceptor<T1, T2>
        where C: 'static + Any,
              F: for<'r> Fn(&mut T1, &mut T2) -> Result<C> + 'static + Send + Sync
    {
        Self::new(move |p1: &mut T1, p2: &mut T2| -> Result<Option<Box<Any>>> {
            Ok(Some(Box::new(try!(constructor(p1, p2)))))
        })
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
        let mut tmp: Option<Arc<Mutex<T1>>> = None;
        mem::swap(&mut tmp,
                  self.d1
                      .get_mut(id)
                      .expect(&format!("expected to find destroy_1 value {:?}", id)));

        truncate_to_used_elements_if_removed_id_is_last(&mut self.d1, id, &mut self.used_size1);
    }

    pub fn destroy_2(&mut self, id: usize) {
        let mut tmp: Option<Arc<Mutex<T2>>> = None;
        mem::swap(&mut tmp,
                  self.d2
                      .get_mut(id)
                      .expect(&format!("expected to find destroy_2 value {:?}", id)));

        truncate_to_used_elements_if_removed_id_is_last(&mut self.d2, id, &mut self.used_size2);
    }
}

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

fn truncate_to_used_elements_if_removed_id_is_last<T>(data: &mut Vec<Option<Arc<Mutex<T>>>>,
                                                      removed_id: usize,
                                                      used_size: &mut usize) {
    if removed_id + 1 != *used_size {
        return;
    }
    while *used_size > 0 {
        *used_size -= 1;
        if *used_size == 0 || !data[*used_size - 1].is_none() {
            break;
        }
    }
    data.truncate(*used_size);
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
    use std::any::Any;
    use std::sync::{Arc, Mutex};
    use super::*;

    fn count_not_none<T>(data: &mut Vec<Option<Arc<Mutex<T>>>>) -> usize {
        data.iter().filter(|v| !v.is_none()).count()
    }

    fn ic_with_val<A: Any, B: Any>() -> Inceptor<A, B> {
        Inceptor::<A, B>::new(move |_a, _b| Ok(Some(Box::new(42))))
    }

    #[test]
    fn should_register_and_destroy_first_arg() {
        let mut ic = ic_with_val::<i32, bool>();
        let (id, instances) = ic.incept_1(Arc::new(Mutex::new(11)))
            .expect("failed to incept first arg");

        assert_eq!(instances.len(), 0);
        assert_eq!(count_not_none(&mut ic.d1), 1);

        ic.destroy_1(id);

        assert_eq!(count_not_none(&mut ic.d1), 0);
    }

    #[test]
    fn should_register_and_destroy_second_arg() {
        let mut ic = ic_with_val::<i32, bool>();
        let (id, instances) = ic.incept_2(Arc::new(Mutex::new(false)))
            .expect("failed to incept first arg");

        assert_eq!(instances.len(), 0);
        assert_eq!(count_not_none(&mut ic.d2), 1);

        ic.destroy_2(id);

        assert_eq!(count_not_none(&mut ic.d2), 0);
    }

    #[test]
    fn should_create_and_destroy_instances_for_all_existing_items() {
        let mut ic = ic_with_val::<i32, bool>();
        let mut value_num_3 = Arc::new(Mutex::new(3));
        let (_, _) = ic.incept_1(Arc::new(Mutex::new(1))).unwrap();
        let (_, _) = ic.incept_1(Arc::new(Mutex::new(2))).unwrap();
        let (id3, _) = ic.incept_1(value_num_3.clone()).unwrap();
        assert_eq!(count_not_none(&mut ic.d1), 3);

        let (other_id, instances) = ic.incept_2(Arc::new(Mutex::new(false))).unwrap();
        assert_eq!(instances.len(), 3);
        assert_eq!(count_not_none(&mut ic.d2), 1);

        // should not be possible to take out value from arc
        value_num_3 = Arc::try_unwrap(value_num_3).unwrap_err();
        // destroying parent should free up the instance that was created using it
        ic.destroy_1(id3);
        assert_eq!(count_not_none(&mut ic.d1), 2);
        // should be possible to take out value from arc
        {
            let val = Arc::try_unwrap(value_num_3).expect("expected arc refcount 1");
            assert_eq!(3, *val.lock().unwrap());
        }
        // memory should be freed
        assert_eq!(ic.d1.len(), 2);

        ic.destroy_2(other_id);
        assert_eq!(count_not_none(&mut ic.d2), 0);

        // memory should be freed
        assert_eq!(ic.d2.len(), 0);
    }
}
