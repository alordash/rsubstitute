use crate::args_matching::arg_info::ArgInfo;

pub enum ArgMatchingResult {
    Ok {
        pub arg_info: ArgInfo,
    },
    Err {
        pub arg_info: ArgInfo,
        pub error_msg: String,
    },
}

impl ArgMatchingResult {
    pub fn ok(arg_info: ArgInfo) -> Self {
        Self::Ok { arg_info }
    }

    pub fn err(arg_info: ArgInfo, error_msg: String) -> Self {
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
}
