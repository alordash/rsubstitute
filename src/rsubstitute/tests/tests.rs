use rsubstitute_proc_macro::mock;

#[mock]
trait Foo {
    fn work(&self, number: i32) -> f32;
}

mod tests {
    use rsubstitute::Times;
    use rsubstitute::prelude::Arg;
    use crate::Foo;
    use super::__rsubstitute_generated::FooMock;

    #[test]
    fn a() {
        let foo_mock = FooMock::new();
        foo_mock.work(Arg::Any).returns(21.0f32);
        
        let result = Foo::work(&foo_mock, 20);
        
        assert_eq!(21.0f32, result);
        foo_mock.work_received(Arg::Eq(20), Times::Once);
    }
}
