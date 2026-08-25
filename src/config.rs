use std::ffi::CString;
use std::path::PathBuf;

use clap::error::Result;

use crate::errors::Errcode;

#[derive(Debug)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,

    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<ContainerOpts, Errcode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();

        let path = argv[0].clone();

        Ok(ContainerOpts {
            path,
            argv,
            uid,
            mount_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_container_opts_from_command() {
        let opts =
            ContainerOpts::new("echo hello".to_string(), 1000, PathBuf::from("/tmp")).unwrap();

        assert_eq!(opts.path, CString::new("echo").unwrap());
        assert_eq!(
            opts.argv,
            vec![
                CString::new("echo").unwrap(),
                CString::new("hello").unwrap(),
            ]
        );
        assert_eq!(opts.uid, 1000);
        assert_eq!(opts.mount_dir, PathBuf::from("/tmp"));
    }
    #[test]
    #[should_panic]
    fn empty_command_panics() {
        ContainerOpts::new("".to_string(), 1000, PathBuf::from("/tmp")).unwrap();
    }
}
