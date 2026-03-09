use rsubstitute::macros::*;

#[mock]
trait Trait<'b> {
    fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args::Arg;

    #[test]
    fn my_test() {
        // Arrange
        let mock = TraitMock::new();
        let v1 = &10;
        let v2 = &20;
        let v3 = &-31;

        let r1 = &111;
        let r2 = &222;
        let r3 = &333;

        mock.setup
            .work(v1)
            .returns(r1)
            .work(v2)
            .returns(r2)
            .and_does(|args| println!("accepted v2, v2 arg = {}", args.0))
            .work(Arg::Is(|x: &&i32| **x < 0))
            .returns(r3);

        // {
        //     let q = 12;
        //     let r = &q;
        //     mock.work(r);
        // }

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3, actual_r3);

        mock.received
            .work(v1, Times::Once)
            .work(v2, Times::Once)
            .work(v3, Times::Once)
            .work(Arg::Any, Times::Exactly(3))
            .no_other_calls();
    }
}

fn main() {}
