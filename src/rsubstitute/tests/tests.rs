use rsubstitute_proc_macro::mock;
use std::rc::Rc;
use std::sync::Arc;

#[mock]
trait Foo {
    fn work(&self, number: i32, string: String, rc: Rc<u8>, arc: Arc<u8>, r#box: Box<u8>) -> f32;
}

mod tests {
    use super::__rsubstitute_generated::FooMock;
    use crate::Foo;
    use rsubstitute::Times;
    use rsubstitute::prelude::Arg;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn a() {
        let foo_mock = FooMock::new();
        let rc = Rc::new(1);
        let arc = Arc::new(2);
        let r#box = Box::new(3);
        foo_mock
            .work(
                Arg::Any,
                Arg::Eq(String::from("amogus")),
                Arg::Any,
                Arg::Any,
                Arg::Any,
            )
            .returns(21.0f32);

        let result = Foo::work(
            &foo_mock,
            20,
            String::from("amogus"),
            rc.clone(),
            arc.clone(),
            r#box,
        );

        assert_eq!(21.0f32, result);
        foo_mock.work_received(
            Arg::Eq(20),
            Arg::Is(|x| x == "amogus"),
            Arg::Eq(rc),
            Arg::Eq(arc),
            Arg::Eq(Box::new(3)),
            Times::Once,
        );
    }
}
