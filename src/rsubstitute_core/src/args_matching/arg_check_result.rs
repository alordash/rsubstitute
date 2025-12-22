use crate::args_matching::arg_info::ArgInfo;

pub enum ArcCheckResult<'a> {
    Ok(ArcCheckResultOk<'a>),
    Err(ArgCheckResultErr<'a>),
}

pub struct ArcCheckResultOk<'a> {
    pub arg_info: ArgInfo<'a>,
}

pub struct ArgCheckResultErr<'a> {
    pub arg_info: ArgInfo<'a>,
    pub error_msg: String,
}

impl<'a> ArcCheckResult<'a> {
    pub fn ok(arg_info: ArgInfo<'a>) -> Self {
        Self::Ok(ArcCheckResultOk { arg_info })
    }

    pub fn err(arg_info: ArgInfo<'a>, error_msg: String) -> Self {
        Self::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        })
    }
    
    pub fn is_ok(&self) -> bool {
        match self {
            ArcCheckResult::Ok(_) => true,
            _ => false
        }
    }

    pub fn as_err(&self) -> Option<&ArgCheckResultErr<'a>> {
        match self {
            ArcCheckResult::Err(result) => Some(result),
            _ => None,
        }
    }
}
