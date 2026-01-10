use rsubstitute::macros::mock;
use rsubstitute_core::args_matching::Arg;

#[mock]
fn global(number: i32) -> String {
    return format!("REAL number is: {number}");
}

#[test]
fn compile() {
    global::setup(Arg::Any).returns(String::from("amogus"));

    let result = global(2);

    assert_eq!("amogus", result);
}
