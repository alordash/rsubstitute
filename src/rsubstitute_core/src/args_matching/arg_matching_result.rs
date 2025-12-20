use crate::args_matching::arg_info::ArgInfo;
use std::fmt::{Debug, Formatter};

pub enum ArgMatchingResult<'a> {
    Ok {
        arg_info: ArgInfo<'a>,
    },
    Err {
        arg_info: ArgInfo<'a>,
        error_msg: String,
    },
}

impl<'a> ArgMatchingResult<'a> {
    pub fn ok(arg_info: ArgInfo<'a>) -> Self {
        Self::Ok { arg_info }
    }

    pub fn err(arg_info: ArgInfo<'a>, error_msg: String) -> Self {
        Self::Err {
            arg_info,
            error_msg,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok { arg_info: _ } => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Self::Err { .. } => true,
            _ => false,
        }
    }
}

impl<'a> Debug for ArgMatchingResult<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgMatchingResult::Ok { arg_info } => write!(
                f,
                "Ok({}: {})",
                arg_info.arg_name(),
                arg_info.arg_type_name()
            ),
            ArgMatchingResult::Err {
                arg_info,
                error_msg,
            } => write!(
                f,
                "Err({}: {}) – {}",
                arg_info.arg_name(),
                arg_info.arg_type_name(),
                error_msg
            ),
        }
    }
}
