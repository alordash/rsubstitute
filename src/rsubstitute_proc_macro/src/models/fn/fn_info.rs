use crate::models::r#fn::*;
use crate::models::*;

pub(crate) struct FnInfo {
    pub syntax: FnSyntax,
    pub call_struct: CallStruct,
    pub args_checker_struct: ArgsCheckerStruct,
}
