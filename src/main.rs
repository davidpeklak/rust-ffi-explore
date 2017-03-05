extern crate libc;

use libc::c_int;

use std::fmt;

#[link(name = "ffiexp")]
extern "C" {
    fn mysocket() -> c_int;
}


extern {
    fn close(fd: c_int) -> c_int;
}

struct SocketDescr {
    fd: c_int
}

impl Drop for SocketDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl fmt::Display for SocketDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}

fn new_socket() -> SocketDescr {
    unsafe {
        let fd = mysocket();
        SocketDescr{ fd: fd}
    }
}

fn main() {
    println!("Hello, world!");
    let x = new_socket();
    println!("x = {}", x);
    let y = new_socket();
    println!("y = {}", y);
}
