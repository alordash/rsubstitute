use rsubstitute_proc_macro::mock;

#[mock]
fn accept_ref(v: &i32) {}

// TODO - support various lifetimes via generics
#[mock]
fn return_ref() -> &'static i32 {
    &1
}

#[mock]
fn accept_ref_return_ref(v: &i32) -> &'static i32 {
    &2
}
