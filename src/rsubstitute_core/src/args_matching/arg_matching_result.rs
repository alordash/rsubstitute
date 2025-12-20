use crate::args_matching::arg_info::ArgInfo;

pub enum ArgMatchingResult<'a> {
    Ok(ArgMatchingResultOk<'a>),
    Err(ArgMatchingResultErr<'a>),
}

pub struct ArgMatchingResultOk<'a> {
    pub arg_info: ArgInfo<'a>,
}

pub struct ArgMatchingResultErr<'a> {
    pub arg_info: ArgInfo<'a>,
    pub error_msg: String,
}

impl<'a> ArgMatchingResult<'a> {
    pub fn ok(arg_info: ArgInfo<'a>) -> Self {
        Self::Ok(ArgMatchingResultOk { arg_info })
    }

    pub fn err(arg_info: ArgInfo<'a>, error_msg: String) -> Self {
        Self::Err(ArgMatchingResultErr {
            arg_info,
            error_msg,
        })
    }
    
    pub fn is_ok(&self) -> bool {
        match self {
            ArgMatchingResult::Ok(_) => true,
            _ => false
        }
    }

    pub fn as_err(&self) -> Option<&ArgMatchingResultErr<'a>> {
        match self {
            ArgMatchingResult::Err(result) => Some(result),
            _ => None,
        }
    }
}
