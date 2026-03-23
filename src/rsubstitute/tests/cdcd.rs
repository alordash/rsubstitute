use rsubstitute::prelude::*;

#[mock]
trait Trait {
    fn anonymous_first<'a>(&self, i: &&'a &&'a &i32);

    fn a_first<'a>(&self, i: &'a &&'a &&'a i32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {}
}
