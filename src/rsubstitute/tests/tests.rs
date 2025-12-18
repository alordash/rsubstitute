use rsubstitute_proc_macro::mock;

#[mock]
trait Foo {
    fn work(&self, number: i32) -> f32;
}

mod tests {
    use crate::Foo;
    use super::__rsubstitute_generated::FooMock;
    
    fn a() {
        let foo_mock = FooMock::new();
        // foo_mock.work(rsubstitute::prelude::Arg::Any);
    }
}