use clap::error::Result;
use nix::sys::wait::waitpid;
use nix::unistd::{Pid, close};

use crate::child::generate_chilld_process;
use crate::cli::Args;
use crate::config::ContainerOpts;
use crate::errors::Errcode;

use nix::sys::utsname::uname;
use std::os::unix::io::RawFd;

pub struct Container {
    config: ContainerOpts,
    sockets: (RawFd, RawFd),
    child_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Container {
            config,
            sockets,
            child_pid: None,
        })
    }
    pub fn create(&mut self) -> Result<(), Errcode> {
        let pid = generate_chilld_process(self.config.clone());
        self.child_pid = Some(pid?);
        log::debug!("Creation finished");
        Ok(())
    }
    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        log::debug!("Cleaning container");
        if let Err(e) = close(self.sockets.0) {
            log::error!("Unable to close write socket: {:?}", e);
            return Err(Errcode::SocketError(3));
        }
        if let Err(e) = close(self.sockets.1) {
            log::error!("Unable to close write socket: {:?}", e);
            return Err(Errcode::SocketError(4));
        }
        Ok(())
    }
}

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = uname();
    let release = host.release();
    log::debug!("Linux release: {}", release);

    if let Ok(version) = scan_fmt!(release, "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::NotSupported(1));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(Errcode::NotSupported(2));
    }

    Ok(())
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("error while creating container {:?}", e);
        return Err(e);
    }
    log::debug!("Container child PID: {:?}", container.child_pid);
    let _ = wait_child(container.child_pid);

    log::debug!("Finished, cleaning and exiting;");

    container.clean_exit()
}
pub fn wait_child(pid: Option<Pid>) -> Result<(), Errcode> {
    if let Some(child_pid) = pid {
        log::debug!("Waiting child pid {:?}", child_pid);
        if let Err(e) = waitpid(child_pid, None) {
            log::error!("Error while for pid to finish: {:?}", e);
            return Err(Errcode::ChildProcessError(1));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn container_create_succeeds() {
        let args = Args {
            command: "echo hello".to_string(),
            uid: 1000,
            debug: true,
            mount_dir: PathBuf::from("/tmp"),
        };

        let mut container = Container::new(args).unwrap();

        assert!(container.create().is_ok());
    }

    #[test]
    fn container_clean_exit_succeeds() {
        let args = Args {
            command: "echo hello".to_string(),
            uid: 1000,
            debug: true,
            mount_dir: PathBuf::from("/tmp"),
        };

        let mut container = Container::new(args).unwrap();

        assert!(container.clean_exit().is_ok());
    }
}
