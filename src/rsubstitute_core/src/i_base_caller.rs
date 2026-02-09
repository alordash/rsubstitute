// TODO - add base callers for `self`, `&mut self`, maybe even `Pin<Self>`?
// because right now you can't mock such trait:
// trait Trait {
//     fn consume(self) -> i32;
//     fn mutate(&mut self);
// }
pub trait IBaseCaller<TCall, TReturnValue> {
    fn call_base(&self, call: TCall) -> TReturnValue;
}