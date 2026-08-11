#![allow(unused)]
use rsubstitute::mock;

#[mock(base)]
fn accept_unsized<T: ?Sized + Clone>(t: &T) {}

#[mock]
struct Struct<T: ?Sized + Clone> {
    t: Box<T>,
}

#[mock(base)]
impl<T: ?Sized + Clone> Struct<T> {
    fn accept<F: ?Sized + Clone>(&self, t: Box<T>, f: &F) {}
}

#[mock(base)]
trait Trait<U: ?Sized + Clone> {
    fn accept<V: ?Sized + Clone>(&self, u: Box<U>, v: &V) {}
}

#[mock(base)]
impl<T: ?Sized + Clone, U: ?Sized + Clone> Trait<U> for Struct<T> {
    fn accept<V: ?Sized + Clone>(&self, u: Box<U>, v: &V) {}
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}
}
