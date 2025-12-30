use rsubstitute::macros::mock;

#[mock]
fn global(number: i32) -> String {
    return format!("REAL number is: {number}");
}

#[test]
fn compile() {
    
}