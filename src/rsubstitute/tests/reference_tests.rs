use rsubstitute_proc_macro::mock;
use std::marker::PhantomData;

#[derive(Default, Debug)]
struct Data<'a, 'b, T1, T2> {
    _phantoms: (
        PhantomData<&'a ()>,
        PhantomData<&'b ()>,
        PhantomData<T1>,
        PhantomData<T2>,
    ),
}

#[mock]
trait Trait<'a, 'b: 'a> {
    fn work<'c, 'd: 'a>(
        &self,
        a: &'a i32,
        b: &'b i32,
        c: &'c i32,
        d: &'d i32,
        axb: &'a &&'b i32,
        cxd: &'c &&'d i32,
        abxbax: &'a &'b &&'b &'a &i32,
        cdxdcx: &'c &'d &&'d &'c &i32,
        abcd: &'a &'b &'c &'d i32,
        xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
        data: Data<
            'a,
            'b,
            &&i32,
            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
        >,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles() {}
}
