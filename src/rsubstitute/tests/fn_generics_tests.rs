use rsubstitute_proc_macro::mock;

#[mock]
fn fsd<T>(value: T) -> T {
    return value;
}

#[test]
fn flex() {}