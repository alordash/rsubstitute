// #![allow(unused_variables)]
// #![allow(non_snake_case)]
// #![allow(unused)]
// 
// use rsubstitute::*;
// #[allow(unused_imports)]
// use rsubstitute_proc_macro::mock;
// use std::ops::Deref;
// 
// #[mock]
// #[derive(Clone)]
// pub struct Struct<S1: Clone>(pub S1);
// 
// #[mock(base)]
// impl<S1: Clone> Struct<S1> {
//     pub fn f(&self, v: i32) -> f32
//     where
//         S1: Clone + Into<i32>,
//     {
//         (v + self.0.clone().into()) as f32
//     }
// 
//     pub fn f_static(v: i32) -> f32 {
//         23f32
//     }
// 
//     pub fn transform(s: Self) -> Self {
//         s
//     }
// }
// 
// fn main() {
//     let s = Struct(10);
//     let mut s_mock = s.mock();
//     s_mock
//         .setup()
//         .f(4)
//         .returns(22f32)
//         .and_does(|_, (v,)| println!("mocked for 4, v = {v}"))
//         .f(3)
//         .call_base()
//         .f(Arg::Any)
//         .returns_with(|(v,)| *v as f32 + 1f32)
//         .and_does(|_, (v,)| println!("mocked for any, v = {v}"));
//     dbg!(s_mock.deref().f(1));
//     dbg!(s_mock.f(1));
//     dbg!(s_mock.f(2));
//     dbg!(s_mock.f(3));
//     dbg!(s_mock.f(4));
//     dbg!(s_mock.f(5));
//     Struct::<i32>::static_setup()
//         .f_static(2)
//         .returns_many([5f32, 1112f32])
//         .and_does(|(v,)| println!("Mocked static, v = {v}"));
//     dbg!(Struct::<i32>::f_static(2));
//     Struct::<i32>::static_received()
//         .f_static(2, Times::Once)
//         .no_other_calls();
//     let s = s_mock.unmock();
// 
//     println!("Done");
// }
// 
// #[mock(base)]
// trait Trait {
//     fn f(v: i32) -> i32;
// }
// use __rsubstitute_generated_TraitMock::*;
// 
// #[mock(base)]
// fn f(v: i32) -> i32 {
//     v + 10
// }

fn main() {}