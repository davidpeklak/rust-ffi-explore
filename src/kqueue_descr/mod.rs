mod token;

use libc::{c_int, kevent};
use super::ffi::*;
use super::{SocketDescr, Accept};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::fmt;
use std::mem::uninitialized;
use std::ptr::{null, null_mut};

pub use self::token::Token;

pub struct KQueueDescr {
    fd: c_int
}

impl Drop for KQueueDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl KQueueDescr {
    pub fn new() -> Result<KQueueDescr, c_int> {
        unsafe {
            let fd = kqueue();
            if fd == -1 {
                Err(my_errno())
            } else {
                Ok(KQueueDescr { fd: fd })
            }
        }
    }

    pub fn register_acc_sock(&mut self, ac: &SocketDescr<Accept>, token: Token) -> Result<(), c_int> {
        unsafe {
            let mut ke: kevent = uninitialized();
            my_ev_set(&mut ke, ac.fd(), From::from(token));
            let rslt = keventfn(self.fd, &mut ke, 1, null_mut(), 0);
            if rslt == 0 {
                Ok(())
            } else {
                Err(my_errno())
            }
        }
    }

    pub fn fetch_event(&mut self) -> Result<Option<c_int>, c_int> {
        unsafe {
            let mut ke: kevent = uninitialized();
            let rslt = keventfn(self.fd, null(), 0, &mut ke, 1);
            if rslt == 0 {
                Ok(None)
            } else if rslt == 1 {
                Ok(Some(ke.data as c_int))
            } else {
                Err(my_errno())
            }
        }
    }

    pub unsafe fn fd(&self) -> c_int {
        self.fd
    }
}

impl fmt::Display for KQueueDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}
