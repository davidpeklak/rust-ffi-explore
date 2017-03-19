use libc::c_int;
use super::ffi::*;
use super::AccSocketDescr;
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::fmt;

pub struct SocketDescr {
    fd: c_int
}

impl Drop for SocketDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl SocketDescr {
    pub fn new() -> Result<SocketDescr, c_int> {
        unsafe {
            let fd = mysocket();
            if fd == -1 {
                Err(my_errno())
            } else {
                Ok(SocketDescr { fd: fd })
            }
        }
    }

    pub fn server_connect(&self, portno: c_int) -> Result<(), c_int> {
        unsafe {
            if serverconnect(self.fd, portno) == -1 {
                Err(my_errno())
            } else {
                Ok(())
            }
        }
    }

    pub fn listen(&self, backlog: c_int) -> Result<(), c_int> {
        unsafe {
            if listen(self.fd, backlog) == -1 {
                Err(my_errno())
            } else {
                Ok(())
            }
        }
    }

    pub fn accept(&self) -> Result<AccSocketDescr, c_int> {
        unsafe {
            let fd = myaccept(self.fd);
            if fd == -1 {
                Err(my_errno())
            } else {
                Ok(AccSocketDescr::from_fd(fd))
            }
        }
    }
}

impl fmt::Display for SocketDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}
