// #![allow(unused_variables)]
// #![allow(non_snake_case)]
// #![allow(unused)]
// 
// use rsubstitute::Mockable;
// #[allow(unused_imports)]
// use rsubstitute_proc_macro::mock;
// use std::ops::Deref;
// use std::thread;
// 
// #[mock]
// trait Trait {
//     fn work() {
//         println!("Default trait work impl");
//     }
// }
// 
// fn f<T: Trait>() {
//     T::work();
// }
// 
// #[mock]
// struct Struct {
//     pub v: i32
// }
// 
// #[mock(base)]
// impl crate::Struct {
//     fn struct_refs(&self) {
//         let s = Struct { v: 1 };
//         let Struct { v: a } = s;
// 
//         let s = crate::Struct { v: 1 };
//         let crate::Struct { v: b } = s;
//     }
// 
//     pub fn f(&self) {
//         println!("Default struct f impl");
//     }
// 
//     pub fn work() {
//         println!("Default struct work impl");
//     }
// }
// 
// fn main() {
//     f::<TraitMock>();
//     TraitMock::static_setup()
//         .work()
//         .does(|_| println!("static Trait::work mocked!"));
//     f::<TraitMock>();
//     f::<TraitMock>();
//     TraitMock::static_setup()
//         .work()
//         .does(|_| println!("new Trait::work mocked!"));
//     f::<TraitMock>();
// 
//     Struct::work();
//     Struct::static_setup()
//         .work()
//         .does(|_| println!("static Struct::work mocked!"));
//     Struct::work();
//     Struct::work();
//     Struct::static_setup()
//         .work()
//         .does(|_| println!("new Struct::work mocked!"));
//     Struct::work();
// 
//     thread::spawn(|| {
//         Struct::work();
//         Struct::static_setup()
//             .work()
//             .does(|_| println!("thread Struct::work mocked!"));
//         Struct::work();
//     })
//         .join();
//     Struct::work();
// 
//     let mut s = Struct.mock();
//     s.f();
//     s.setup().f().does(|_, _| println!("mocked Struct::f!"));
//     // s.setup().f().does(|_| println!("not mock arg"));
//     s.f();
// 
//     println!("Done");
// }

fn main() {}