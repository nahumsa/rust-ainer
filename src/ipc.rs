use std::os::unix::io::RawFd;

use clap::error::Result;
use nix::sys::socket::{MsgFlags, recv, send};

use crate::errors::Errcode;

#[allow(dead_code)]
pub fn send_boolean(fd: RawFd, boolean: bool) -> Result<(), Errcode> {
    let data: [u8; 1] = [boolean.into()];

    if let Err(e) = send(fd, &data, MsgFlags::empty()) {
        log::error!("Cannot send boolean through the socket: {:?}", e);
        return Err(Errcode::SocketError(1));
    };
    Ok(())
}

#[allow(dead_code)]
pub fn recv_boolean(fd: RawFd) -> Result<bool, Errcode> {
    let mut data: [u8; 1] = [0];

    if let Err(e) = recv(fd, &mut data, MsgFlags::empty()) {
        log::error!("Cannot receive boolean from socket: {:?}", e);
        return Err(Errcode::SocketError(2));
    };
    Ok(data[0] == 1)
}
