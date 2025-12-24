use crate::args_matching::arg_info::ArgInfo;

pub enum ArgCheckResult<'a> {
    Ok(ArgCheckResultOk<'a>),
    Err(ArgCheckResultErr<'a>),
}

pub struct ArgCheckResultOk<'a> {
    pub arg_info: ArgInfo<'a>,
}

pub struct ArgCheckResultErr<'a> {
    pub arg_info: ArgInfo<'a>,
    pub error_msg: String,
}

impl<'a> ArgCheckResult<'a> {
    pub fn ok(arg_info: ArgInfo<'a>) -> Self {
        Self::Ok(ArgCheckResultOk { arg_info })
    }

    pub fn err(arg_info: ArgInfo<'a>, error_msg: String) -> Self {
        Self::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        })
    }
    
    pub fn is_ok(&self) -> bool {
        match self {
            ArgCheckResult::Ok(_) => true,
            _ => false
        }
    }

    pub fn as_err(&self) -> Option<&ArgCheckResultErr<'a>> {
        match self {
            ArgCheckResult::Err(result) => Some(result),
            _ => None,
        }
    }
}
