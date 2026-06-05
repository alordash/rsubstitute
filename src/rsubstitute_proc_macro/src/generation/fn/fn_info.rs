use super::*;
use crate::preparation::r#fn::*;

pub(crate) struct FnInfo {
    pub syntax: FnSyntax,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
}
