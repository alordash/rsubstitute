use rsubstitute::macros::mock;
use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;

#[mock]
fn f(number: i32) -> String {
    return format!("REAL number is: {number}");
}

#[test]
fn f_ok() {
    // Arrange
    f::setup(Arg::Any).returns(String::from("amogus"));

    // Act
    let result = f(2);

    // Assert
    assert_eq!("amogus", result);
    f::received(Arg::Eq(2), Times::Once);
}
