use libc::{c_int, ssize_t};
use ffi::*;
use std::fmt;

pub struct AccSocketDescr {
    fd: c_int
}

impl Drop for AccSocketDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl AccSocketDescr {
    pub unsafe fn from_fd(fd: c_int) -> AccSocketDescr {
        AccSocketDescr { fd: fd }
    }

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

impl fmt::Display for AccSocketDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}
