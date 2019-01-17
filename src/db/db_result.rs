#[derive(Debug)]
pub enum DbResult<T> {
    Many(Vec<T>),
    One(T)
}
