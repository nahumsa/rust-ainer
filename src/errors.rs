use std::fmt;
use std::process::exit;

use crate::container::MINIMAL_KERNEL_VERSION;
use clap::error::Result;

#[derive(Debug)]
pub enum Errcode {
    ContainerError(u8),
    NotSupported(u8),
    ArgumentInvalid(&'static str),
}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::NotSupported(errtype) => match errtype {
                0 => write!(
                    f,
                    "Minimal kernel version required: {}",
                    MINIMAL_KERNEL_VERSION
                ),
                1 => write!(f, "Only x86_64 architecture is supported"),
                _ => write!(f, "{:?} (unknown id)", self),
            },
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for Errcode {}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit \n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
