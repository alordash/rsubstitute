use rsubstitute::*;

mod unit_mod {
    use super::*;

    #[mock]
    pub struct Unit;

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
}

#[mock(base)]
impl unit_mod::Unit {
    fn path(&self) {
        let unit = unit_mod::Unit;
        let unit_mod::Unit = unit;
        let s = Self;
        let Self = s;
    }

    fn static_path() {
        let unit = unit_mod::Unit;
        let unit_mod::Unit = unit;
        let s = Self;
        let Self = s;
    }
}

// mod named_mod {
//     use rsubstitute_proc_macro::mock;
// 
//     #[mock]
//     pub struct Named {
//         pub v: i32,
//     }
// 
//     #[mock(base)]
//     impl Named {
//         fn ident(&self) {
//             let unit = Named { v: 1 };
//             let Named { v: a } = unit;
//             let s = Self;
//             let Self = s;
//         }
// 
//         fn static_ident() {
//             let unit = Named { v: 1 };
//             let Named { v: a } = unit;
//             let s = Self;
//             let Self = s;
//         }
//     }
// }
// 
// #[mock(base)]
// impl named_mod::Named {
//     fn path(&self) {
//         let unit = named_mod::Named { v: 1 };
//         let named_mod::Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// 
//     fn static_path() {
//         let unit = named_mod::Named { v: 1 };
//         let named_mod::Named { v: a } = unit;
//         let s = Self;
//         let Self = s;
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {
        let s = unit_mod::Unit {
            __rs_data: Default::default(),
        };
    }
}
