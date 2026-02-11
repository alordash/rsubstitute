use rsubstitute::macros::mock;

#[mock]
trait Trait<'a> {
    fn accept_ref(&self, r: &&'a& i32);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}
}
