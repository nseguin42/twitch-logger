pub trait ValueLogger<T> {
    fn log(&mut self, value: T);
}
