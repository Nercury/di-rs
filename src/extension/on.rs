#[cfg(test)]
mod test {
    use Deps;
    use std::sync::{ Arc, Mutex };

    #[derive(Clone)]
    struct A(String);

    #[derive(Clone)]
    struct B(String);

    #[derive(Clone)]
    struct C(String);

    #[test]
    fn creates_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_b_ref = Arc::new(Mutex::new(None));

        deps.chainable({
            let created_b_ref = created_b_ref.clone();
            move |_: &Deps, a: &mut A| {
                let b = B([&a.0[..], "+B"].concat());
                *created_b_ref.lock().unwrap() = Some(b.clone());
                Ok(b)
            }
        });

        deps.chain(A("Hello".into())).unwrap();

        assert_eq!("Hello+B", (*created_b_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_dependency_of_dependency() {
        let mut deps = Deps::new();

        // here we want to know what is the state of dependency in closure, hence
        // shared mutable reference to it
        let created_c_ref = Arc::new(Mutex::new(None));

        deps.chainable(|_: &Deps, a: &mut A| Ok(B([&a.0[..], "+B"].concat())));

        deps.chainable({
            let created_c_ref = created_c_ref.clone();
            move |_: &Deps, b: &mut B| {
                let c = C([&b.0[..], "+C"].concat());
                *created_c_ref.lock().unwrap() = Some(c.clone());
                Ok(c)
            }
        });

        deps.chain(A("Hello".into())).unwrap();

        assert_eq!("Hello+B+C", (*created_c_ref.lock().unwrap()).clone().unwrap().0);
    }

    #[test]
    fn creates_mutable_dependency() {
        let mut deps = Deps::new();

        deps.chainable(|_: &Deps, a: &mut A| {*a = A("Hi!".into()); Ok(())});

        let a = deps.chain(A("Hello".into())).unwrap();

        assert_eq!("Hi!", a.obj.0);
    }
}
