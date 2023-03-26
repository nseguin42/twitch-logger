pub trait ValueLogger<T> {
    fn log(&self, value: T);
}

struct AdapterFn<T, S>(Box<dyn Fn(S) -> T>);

impl<T, S> AdapterFn<T, S>
where
    T: 'static,
    S: 'static,
{
    fn new(f: fn(S) -> T) -> Self {
        Self(Box::new(f))
    }

    fn adapt(&self, value: S) -> T {
        (self.0)(value)
    }
}
