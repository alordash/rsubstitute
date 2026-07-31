use rsubstitute::*;

#[mock]
struct Unit;

#[mock(base)]
impl Unit {
    fn ident(&self) {
        let unit = Unit;
        let Unit = unit;
        let s = Self;
        let Self = s;
    }

    fn static_ident() {
        let unit = Unit;
        let Unit = unit;
        let s = Self;
        let Self = s;
    }
}

// #[mock(base)]
// impl crate::Unit {
//     fn path(&self) {
//         let unit = crate::Unit;
//         let crate::Unit = unit;
//         let s = Self;
//         let Self = s;
//     }
// 
//     fn static_path() {
//         let unit = crate::Unit;
//         let crate::Unit = unit;
//         let s = Self;
//         let Self = s;
//     }
// }
// 
// #[mock]
// struct Named {
//     pub v: i32,
// }
// 
// #[mock(base)]
// impl Named {
//     fn ident(&self) {
//         let unit = Named { v: 1 };
//         let Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// 
//     fn static_ident() {
//         let unit = Named { v: 1 };
//         let Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// }
// 
// #[mock(base)]
// impl crate::Named {
//     fn path(&self) {
//         let unit = crate::Named { v: 1 };
//         let crate::Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// 
//     fn static_path() {
//         let unit = crate::Named { v: 1 };
//         let crate::Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {}
}
