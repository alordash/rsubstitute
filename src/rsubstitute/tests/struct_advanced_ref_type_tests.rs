use rsubstitute::macros::*;

mocked! {
    struct Struct;

    impl Struct {
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn flex() {}

    #[test]
    fn compile() {}
}
