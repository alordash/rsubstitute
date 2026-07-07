#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

#[allow(unused_imports)]
use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn f(&self);
    fn f_static();
}

fn main() {
    let mut mock = TraitMock::new();
    mock.setup().f().does(|_| println!("mocked callback"));
    mock.f();
    TraitMock::static_setup()
        .f_static()
        .does(|_| println!("mocked static call"));
    TraitMock::f_static();

    println!("Done");
}
