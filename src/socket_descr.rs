use libc::{c_int, ssize_t};
use super::ffi::*;
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::fmt;
use std::marker::PhantomData;

pub enum New {}
pub enum Listen {}
pub enum Accept {}

pub struct SocketDescr<T> {
    fd: c_int,
    phantom: PhantomData<T>
}

impl<T> Drop for SocketDescr<T> {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl<T> fmt::Display for SocketDescr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}

impl SocketDescr<New> {
    pub fn new() -> Result<SocketDescr<New>, c_int> {
        unsafe {
            let fd = mysocket();
            if fd == -1 {
                Err(my_errno())
            } else {
                Ok(SocketDescr::<New> { fd: fd, phantom: PhantomData })
            }
        }
    }
}

impl SocketDescr<New> {

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

    pub fn accept(&self) -> Result<SocketDescr<Accept>, c_int> {
        unsafe {
            let fd = myaccept(self.fd);
            if fd == -1 {
                Err(my_errno())
            } else {
                Ok(SocketDescr::<Accept> { fd: fd, phantom: PhantomData })
            }
        }
    }
}

impl SocketDescr<Accept> {
    pub fn receive(&self, buf: &mut [u8]) -> Result<ssize_t, c_int> {
        unsafe {
            let rslt = recv(self.fd, &mut buf[0], buf.len() as c_int, 0);
            if rslt == -1 {
                Err(my_errno())
            } else {
                Ok(rslt)
            }
        }
    }

    pub unsafe fn fd(&self) -> c_int {
        self.fd
    }
}
