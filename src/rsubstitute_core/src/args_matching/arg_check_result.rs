use crate::args_matching::arg_info::ArgInfo;

pub enum ArgCheckResult {
    Ok(ArgCheckResultOk),
    Err(ArgCheckResultErr),
}

pub struct ArgCheckResultOk {
    pub arg_info: ArgInfo,
}

pub struct ArgCheckResultErr {
    pub arg_info: ArgInfo,
    pub error_msg: String,
}

impl ArgCheckResult {
    pub fn ok(arg_info: ArgInfo) -> Self {
        Self::Ok(ArgCheckResultOk { arg_info })
    }

    pub fn err(arg_info: ArgInfo, error_msg: String) -> Self {
        Self::Err(ArgCheckResultErr {
            arg_info,
            error_msg,
        })
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ArgCheckResult::Ok(_) => true,
            _ => false,
        }
    }

    pub fn as_err(&self) -> Option<&ArgCheckResultErr> {
        match self {
            ArgCheckResult::Err(result) => Some(result),
            _ => None,
        }
    }
}
