use libc::{c_int, kevent};
use super::ffi::*;
use super::AccSocketDescr;
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::fmt;
use std::mem::uninitialized;

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

    pub fn kevent(&self, kep: &mut KEventParam) -> c_int {
        unsafe { keventfn(self.fd, &kep.ch_list[0], 1, &mut kep.ev_list[0], 1) }
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

pub struct KEventParam {
    ch_list: [kevent; 1],
    ev_list: [kevent; 1]
}

impl KEventParam {
    pub fn new(ac: &AccSocketDescr) -> KEventParam {
        let mut rslt = KEventParam {
            ch_list: unsafe { [uninitialized(); 1] },
            ev_list: unsafe { [uninitialized(); 1] }
        };

        unsafe { my_ev_set(&mut rslt.ch_list[0], ac.fd()) };

        rslt
    }
}