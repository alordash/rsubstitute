pub enum Arg<T> {
    Any,
    Is(fn(T) -> bool),
    Eq(T),
}
