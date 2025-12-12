pub struct FnConfig<TArgsMatcher, TReturnValue> {
    args_matcher: TArgsMatcher,
    return_value: Option<TReturnValue>,
}

impl<TArgsMatcher, TReturnValue> FnConfig<TArgsMatcher, TReturnValue> {
    pub fn new(args_matcher: TArgsMatcher) -> Self {
        FnConfig {
            args_matcher,
            return_value: None,
        }
    }

    pub fn set_return_value(&mut self, return_value: TReturnValue) {
        self.return_value = Some(return_value);
    }
}