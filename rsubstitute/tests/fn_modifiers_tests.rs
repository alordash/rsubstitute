use rsubstitute::*;

async fn async_dep() {}
unsafe fn unsafe_dep() {}
extern "C" fn extern_dep() {}

#[allow(unused)]
#[mock]
async fn async_fn() {
    async_dep().await
}
#[allow(unused)]
#[mock(base)]
async fn async_fn_base() {
    async_dep().await
}

#[allow(unused)]
#[mock]
unsafe fn unsafe_fn() {
    unsafe_dep()
}
#[allow(unused)]
#[mock(base)]
unsafe fn unsafe_fn_base() {
    unsafe_dep()
}

#[allow(unused)]
#[mock]
extern "C" fn extern_fn() {
    extern_dep()
}
#[allow(unused)]
#[mock(base)]
extern "C" fn extern_fn_base() {
    extern_dep()
}

#[mock(base)]
#[allow(unused)]
trait Trait {
    async fn async_fn(&self);
    async fn async_fn_base(&self) {
        async_dep().await
    }
    unsafe fn unsafe_fn(&self);
    unsafe fn unsafe_fn_base(&self) {
        unsafe { unsafe_dep() }
    }
    extern "C" fn extern_fn(&self);
    extern "C" fn extern_fn_base(&self) {
        extern_dep()
    }
}

#[mock]
struct Struct;

#[mock]
#[allow(unused)]
impl Struct {
    async fn async_fn(&self) {
        async_dep().await
    }
    unsafe fn unsafe_fn(&self) {
        unsafe_dep()
    }
    extern "C" fn extern_fn(&self) {
        extern_dep()
    }
}
#[mock(base)]
#[allow(unused)]
impl Struct {
    async fn async_fn_base(&self) {
        async_dep().await
    }
    unsafe fn unsafe_fn_base(&self) {
        unsafe_dep()
    }
    extern "C" fn extern_fn_base(&self) {
        extern_dep()
    }
}

#[mock(base)]
impl Trait for Struct {
    async fn async_fn(&self) {
        self.async_fn().await
    }
    async fn async_fn_base(&self) {
        self.async_fn_base().await
    }
    extern "C" fn extern_fn(&self) {
        self.extern_fn()
    }
    extern "C" fn extern_fn_base(&self) {
        self.extern_fn_base()
    }
    unsafe fn unsafe_fn_base(&self) {
        unsafe { self.unsafe_fn_base() }
    }
    unsafe fn unsafe_fn(&self) {
        unsafe { self.unsafe_fn() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
