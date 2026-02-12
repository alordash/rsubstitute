use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn mutate(&mut self);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn flex() {
        let mut mock = TraitMock::new();

        mock.setup().mutate().does(|| println!("mutating!"));

        mock.mutate();
    }

    #[test]
    fn compile() {}
}
