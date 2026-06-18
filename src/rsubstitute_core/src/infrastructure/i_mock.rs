pub trait IMock<TMock>: Sized {
    fn get_boxed_mocked(&mut self) -> &mut TMock;

    fn drop_boxed_mocked(&mut self) {
        let mocked = self.get_boxed_mocked();
        // SAFETY: this frees memory leaked from `IMocked::mock` method.
        unsafe {
            let _ = Box::from_raw(mocked as *mut _);
        }
    }
}
