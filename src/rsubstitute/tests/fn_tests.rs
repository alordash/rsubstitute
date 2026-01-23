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
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn f_Ok() {
        // Arrange
        f::setup(Arg::Any).returns(String::from("amogus"));

        // Act
        let result = f(2);

        // Assert
        assert_eq!("amogus", result);
        f::received(Arg::Eq(2), Times::Once).no_other_calls();
    }

    #[test]
    fn g_Ok() {
        // Arrange
        g::setup(Arg::Eq(1)).returns(String::from("g1"));

        // Act
        let result = g(1);

        // Assert
        assert_eq!("g1", result);
        g::received(Arg::Eq(1), Times::Once).no_other_calls();
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
        g::received(Arg::Eq(3), Times::Once).no_other_calls();
        f::received(Arg::Eq(3), Times::Once).no_other_calls();
    }
}
