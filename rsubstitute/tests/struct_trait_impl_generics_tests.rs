use rsubstitute::*;

#[allow(unused)]
trait Trait<T> {
    fn work(&self, t: T) -> T;
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {}

#[mock(base)]
impl<T: Clone> Trait<T> for Struct {
    fn work(&self, t: T) -> T {
        t
    }
}

mod tests {
    #[test]
    fn compile() {}
}
