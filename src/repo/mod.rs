pub trait CrudRepository<T> {
    fn save(&self, t: T) -> T;
    fn findById(&self, id: i32) -> T;
    fn delete(&self, id: i32);
}
