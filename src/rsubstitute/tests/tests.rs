use rsubstitute_proc_macro::mock;

#[mock]
trait Foo {
    fn work(&self, number: i32) -> f32;
}

mod tests {
    use super::*;
    
    fn a() {
        
    }
}