use std::fmt::Debug;

use rsubstitute::macros::*;
#[mock(base)]
#[allow(unused)]
trait Trait<'a, T1> {
    fn work<T2: Clone, T3: Default, const B: bool, const N: usize>(&self, t1: T1, t2: &'a T2) -> T3
    where
        T1: Clone,
    {
        println!(
            "calling base ftw. SizeOf(T1) = {}, SizeOf(T2) = {}",
            size_of::<T1>(),
            size_of::<T2>()
        );
        dbg!(core::ptr::from_ref(t2));
        return T3::default();
    }
}

#[derive(Clone, Debug)]
struct Foo {
    amogus: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args::Arg;

    #[test]
    fn my_test() {
        let mock = TraitMock::new();

        let v2 = 22;
        let v3 = 33;
        let v4 = [10; 5];

        mock.setup
            .work::<_, i32, true, 2>(10, &"amogus")
            .call_base()
            .work::<_, _, true, 4>(10, &"amogus")
            .returns(v2)
            .and_does(|_| println!("I don't care what was received"))
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v3)
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v4)
            .work::<Foo, _, false, 2>(23, Arg::Any)
            .returns(22);

        let av3 = mock.work::<_, i32, false, 2>(10, &"amogus");
        let av2 = mock.work::<_, i32, true, 4>(10, &"amogus");
        let av1 = mock.work::<_, i32, true, 2>(10, &"amogus");
        let av4 = mock.work::<_, [i32; 5], false, 2>(10, &"amogus");
        let av5 = mock.work::<_, i32, false, 2>(23, &Foo { amogus: 53.2f32 });

        assert_eq!(i32::default(), av1);
        assert_eq!(v2, av2);
        assert_eq!(v3, av3);
        assert_eq!(v4, av4);

        mock.received
            .work::<_, i32, true, 2>(10, &"amogus", Times::Once)
            .work::<_, i32, true, 4>(10, &"amogus", Times::Once)
            .work::<_, i32, false, 2>(10, &"amogus", Times::Once)
            .work::<_, [i32; 5], false, 2>(10, &"amogus", Times::Once)
            .work::<_, i32, true, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 4>(11, &"amogus", Times::Never)
            .work::<_, i32, false, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 2>(10, &true, Times::Never)
            .work::<Foo, i32, false, 2>(
                23,
                Arg::Is(|foo: &&Foo| foo.amogus == 53.2f32),
                Times::Once,
            )
            .no_other_calls();
    }
}

fn main() {}
