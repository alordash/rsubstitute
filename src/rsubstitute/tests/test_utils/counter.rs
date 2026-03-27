use std::sync::atomic::{AtomicUsize, Ordering};

// TODO - reuse
// Helper struct for callbacks verification.
// Leaking for ability to `Copy` so no need to create clones for moving them in closures.
#[derive(Copy, Clone)]
pub struct Counter(&'static AtomicUsize);
impl Counter {
    pub fn new() -> Self {
        Self(Box::leak(Box::new(AtomicUsize::new(0))))
    }
    pub fn inc(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    pub fn double_inc(&self) {
        self.0.fetch_add(2, Ordering::Relaxed);
    }
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

mod tests {

    #[test]
    fn compile() {}
}