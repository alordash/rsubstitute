use rsubstitute_proc_macro::mock;

#[mock]
trait TestTrait {
    fn f_accept_value(&self, v: i32);

    fn f_return_value(&self) -> i32;

    fn f_accept_value_return_value(&self, v: i32) -> f32;
    
    fn f_accept_two_values(&self, v1: i32, v2: f32);
    
    fn f_accept_two_values_return_value(&self, v1: i32, v2: f32) -> String;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::__rsubstitute_generated_TestTrait::*;
    
    
}