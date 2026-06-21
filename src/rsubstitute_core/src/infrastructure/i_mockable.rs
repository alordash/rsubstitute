pub trait IMockable: Sized {
    type Mock;

    // TODO: in case some user's mockable struct can't be boxed try to replace it with unsafe
    // by creating mock from mut ref and `Box::leak`ing
    fn convert(boxed_mockable: Box<Self>) -> Self::Mock;

    fn mock<'__rs>(self) -> Self::Mock {
        let boxed_mockable = Box::new(self);
        let mock = Self::convert(boxed_mockable);
        return mock;
    }
}
