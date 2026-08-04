use std::fmt;
use std::process::exit;

use clap::error::Result;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
}

impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
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
