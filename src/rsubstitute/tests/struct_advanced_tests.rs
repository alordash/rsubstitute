use rsubstitute::macros::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

mocked! {
    struct Struct;

    impl Struct {
        pub fn new() -> Self {
            Self
        }

        fn fooo(&mut self, Foo { mut number }: Foo, mut qq: &mut &mut &&& &mut i32) {
            println!("number: {number:?}")
        }
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
