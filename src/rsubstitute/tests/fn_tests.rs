#![allow(unused_variables)]
#![allow(non_snake_case)]

use rsubstitute::macros::mock;

#[mock]
fn f(number: i32) -> String {
    return format!("REAL number is: {number}");
}

#[mock]
fn g(number: i32) -> String {
    return f(number);
}

mod tests {
    use crate::{f, g};
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn f_Ok() {
        // Arrange
        f::setup(Arg::Any).returns(String::from("amogus"));

        // Act
        let result = f(2);

        // Assert
        assert_eq!("amogus", result);
        f::received(Arg::Eq(2), Times::Once).only();
    }

    #[test]
    fn g_Ok() {
        // Arrange
        g::setup(Arg::Eq(1)).returns(String::from("g1"));

        // Act
        let result = g(1);

        // Assert
        assert_eq!("g1", result);
        g::received(Arg::Eq(1), Times::Once).only();
    }

    #[test]
    fn g_CallBase_Ok() {
        // Arrange
        g::setup(Arg::Any).call_base();
        f::setup(Arg::Any).returns(String::from("quo vadis"));

        // Act
        let result = g(3);

        // Assert
        assert_eq!("quo vadis", result);
        g::received(Arg::Eq(3), Times::Once).only();
        f::received(Arg::Eq(3), Times::Once).only();
    }
}
