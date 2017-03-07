extern crate libc;

use libc::{c_int, c_uint, ssize_t};
use std::result::Result;
use std::result::Result::{Ok, Err};

use std::fmt;

extern "C" {
    fn mysocket() -> c_int;
    fn serverconnect(sockfd: c_int, portno: c_int) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn myaccept(sockfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn recv(sockfd: c_int, buf: *mut u8, len: c_int, flags: c_uint) -> ssize_t;
    fn my_errno() -> c_int;
}

struct SocketDescr {
    fd: c_int
}

struct AccSocketDescr {
    fd: c_int
}

impl Drop for SocketDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl SocketDescr {
    fn new() -> Result<SocketDescr, c_int> {
        unsafe {
            let fd = mysocket();
            if fd == -1 {
                Err(my_errno())
            }
            else {
                Ok(SocketDescr{fd: fd})
            }
        }
    }

    fn server_connect(&self, portno: c_int) -> Result<(), c_int> {
        unsafe {
            if serverconnect(self.fd, portno) == -1 {
                Err(my_errno())
            }
            else {
                Ok(())
            }

        }
    }

    fn listen(&self, backlog: c_int) -> Result<(), c_int> {
        unsafe {
            if listen(self.fd, backlog) == -1 {
                Err(my_errno())
            }
            else {
                Ok(())
            }
        }
    }

    fn accept(&self) -> Result<AccSocketDescr, c_int> {
        unsafe {
            let fd =  myaccept(self.fd);
            if fd == -1 {
                Err(my_errno())
            }
            else {
                Ok(AccSocketDescr{fd: fd})
            }
        }
    }
}

impl fmt::Display for SocketDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}

impl Drop for AccSocketDescr {
    fn drop(&mut self) {
        println!("Dropping {}", &self);
        unsafe { close(self.fd) };
    }
}

impl AccSocketDescr {
    fn receive(&self, buf: &mut [u8]) -> Result<ssize_t, c_int> {
        unsafe {
            let rslt = recv(self.fd, &mut buf[0], buf.len() as c_int, 0);
            if rslt == -1 {
                Err(my_errno())
            }
            else {
                Ok(rslt)
            }
        }
    }
}

impl fmt::Display for AccSocketDescr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fd: {})", self.fd)
    }
}

fn main() {
    let s = SocketDescr::new()
        .expect("Failed to create socket");
    s.server_connect(3128)
        .expect("Failed to connect to socket");
    s.listen(5)
        .expect("Failed to listen to socket");
    let ac = s.accept()
         .expect("Failed to accept connection");

    let mut buf = [0u8; 16];

    loop {
        let length = ac.receive(&mut buf)
            .expect("Failed to receive on socket");

        if length == 0 {
            break
        }

        let s = std::str::from_utf8(&buf).unwrap();

        println!("Received: {}", s)
    }

}
